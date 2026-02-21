mod network;
mod shell;
mod sysinfo;

use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::Duration;

use network::{Client, MultiplexManager};
use shared::packets::{Heartbeat, Packet};

#[allow(clippy::too_many_lines)]
fn main() {
    let audit_id = std::env::var("AGENT_AUDIT_ID").expect("AGENT_AUDIT_ID must be set");
    let agent_token = std::env::var("AGENT_TOKEN").expect("AGENT_TOKEN must be set");
    let server_addr = std::env::var("AGENT_SERVER_ADDR").expect("AGENT_SERVER_ADDR must be set");

    let mut client = loop {
        if let Ok(client) = Client::new(&server_addr) {
            break client;
        }
        eprintln!("Failed to connect to server, retrying in 5 seconds..");
        std::thread::sleep(Duration::from_secs(5));
    };
    println!("Connected to server successfully!");

    // Authenticate with audit credentials
    match client.authenticate(&audit_id, &agent_token) {
        Ok(()) => println!("Authentication successful for audit {audit_id}"),
        Err(e) => {
            eprintln!("Authentication failed: {e}");
            return;
        }
    }

    let manager = Arc::new(MultiplexManager::new(client));
    let shutdown_flag = manager.shutdown_flag();

    let _receive_thread = manager.start();

    println!("Multiplex manager started, waiting for streams from server...");

    // Heartbeat thread: send every 10 seconds
    let hb_manager = manager.clone();
    let hb_shutdown = shutdown_flag.clone();
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(Duration::from_secs(10));
            if hb_shutdown.load(Ordering::Relaxed) {
                break;
            }
            let hb = Heartbeat::new();
            match hb.serialize() {
                Ok(buf) => {
                    if let Err(e) = hb_manager.send_control_packet(&buf) {
                        eprintln!("Failed to send heartbeat: {e}");
                        break;
                    }
                }
                Err(e) => eprintln!("Failed to serialize heartbeat: {e}"),
            }
        }
    });

    // System info thread: send immediately, then every 60 seconds
    let si_manager = manager.clone();
    let si_shutdown = shutdown_flag.clone();
    std::thread::spawn(move || {
        let send_info = |mgr: &Arc<MultiplexManager>| {
            let info = sysinfo::collect_system_info();
            match info.serialize() {
                Ok(buf) => {
                    if let Err(e) = mgr.send_control_packet(&buf) {
                        eprintln!("Failed to send system info: {e}");
                        return false;
                    }
                    println!("System info sent to server");
                }
                Err(e) => eprintln!("Failed to serialize system info: {e}"),
            }
            true
        };

        if !send_info(&si_manager) {
            return;
        }
        loop {
            std::thread::sleep(Duration::from_secs(60));
            if si_shutdown.load(Ordering::Relaxed) {
                break;
            }
            if !send_info(&si_manager) {
                break;
            }
        }
    });

    loop {
        if shutdown_flag.load(Ordering::Relaxed) {
            println!("Shutdown signal received, exiting...");
            break;
        }

        match manager.accept_stream() {
            Ok(stream) => {
                println!("Stream opened by server: stream_id={}", stream.id());

                std::thread::spawn(move || {
                    loop {
                        match stream.receive() {
                            Ok(data) => {
                                if data.is_empty() {
                                    println!("Stream {} closed", stream.id());
                                    break;
                                }
                                println!(
                                    "Stream {}: received {} bytes: {:?}",
                                    stream.id(),
                                    data.len(),
                                    String::from_utf8_lossy(&data)
                                );

                                if let Err(e) = stream.send_bytes(&data) {
                                    eprintln!("Failed to send on stream {}: {}", stream.id(), e);
                                    break;
                                }
                                println!("Stream {}: sent echo", stream.id());
                            }
                            Err(e) => {
                                eprintln!("Stream {} receive error: {}", stream.id(), e);
                                break;
                            }
                        }
                    }
                });
            }
            Err(e) => {
                if shutdown_flag.load(Ordering::Relaxed) {
                    println!("Shutdown signal received, exiting...");
                } else {
                    eprintln!("Failed to accept stream: {e}");
                }
                break;
            }
        }
    }
}
