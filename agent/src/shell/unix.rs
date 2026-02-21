//! Unix-specific shell implementation (Linux, macOS)

use super::ShellSession;
use shared::error::NetworkError;
use shared::packets::ShellType;
use std::process::{Command, Stdio};

/// Detect the best available shell on Unix systems
fn detect_shell() -> &'static str {
    // Prefer bash, fallback to sh
    if std::path::Path::new("/bin/bash").exists() {
        "/bin/bash"
    } else if std::path::Path::new("/bin/sh").exists() {
        "/bin/sh"
    } else {
        // Last resort
        "sh"
    }
}

/// Spawn a shell process on Unix systems
pub fn spawn_shell(shell_type: ShellType) -> Result<ShellSession, NetworkError> {
    let shell_path = match shell_type {
        ShellType::Auto => detect_shell(),
        ShellType::Bash => "/bin/bash",
        ShellType::Sh => "/bin/sh",
        ShellType::Cmd | ShellType::PowerShell => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "Windows shells not available on Unix",
            )
            .into());
        }
    };

    eprintln!("Spawning Unix shell: {shell_path}");

    let mut process = Command::new(shell_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdin = process.stdin.take().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::BrokenPipe, "Failed to capture stdin")
    })?;

    let stdout = process.stdout.take().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::BrokenPipe, "Failed to capture stdout")
    })?;

    Ok(ShellSession {
        process: Some(process),
        stdin: Some(stdin),
        stdout: Some(stdout),
    })
}
