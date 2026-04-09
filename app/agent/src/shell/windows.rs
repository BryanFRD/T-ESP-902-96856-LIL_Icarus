//! Windows-specific shell implementation

use super::ShellSession;
use shared::error::NetworkError;
use shared::packets::ShellType;
use std::process::{Command, Stdio};

/// Detect the best available shell on Windows
fn detect_shell() -> &'static str {
    // Prefer PowerShell, fallback to cmd.exe
    if which::which("powershell.exe").is_ok() {
        "powershell.exe"
    } else {
        "cmd.exe"
    }
}

/// Spawn a shell process on Windows
pub fn spawn_shell(shell_type: ShellType) -> Result<ShellSession, NetworkError> {
    let (shell_cmd, args) = match shell_type {
        ShellType::Auto => {
            let shell = detect_shell();
            if shell == "powershell.exe" {
                ("powershell.exe", vec!["-NoLogo", "-NoProfile"])
            } else {
                ("cmd.exe", vec![])
            }
        }
        ShellType::PowerShell => ("powershell.exe", vec!["-NoLogo", "-NoProfile"]),
        ShellType::Cmd => ("cmd.exe", vec![]),
        ShellType::Bash | ShellType::Sh => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "Unix shells not available on Windows",
            )
            .into());
        }
    };

    eprintln!("Spawning Windows shell: {shell_cmd}");

    let mut command = Command::new(shell_cmd);
    for arg in args {
        command.arg(arg);
    }

    let mut process = command
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
