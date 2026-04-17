//! Unix-specific shell implementation (Linux, macOS)

use super::ShellSession;
use shared::error::NetworkError;
use shared::packets::ShellType;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};

/// Detect the best available shell on Unix systems
fn detect_shell() -> &'static str {
    if std::path::Path::new("/bin/bash").exists() {
        "/bin/bash"
    } else if std::path::Path::new("/bin/sh").exists() {
        "/bin/sh"
    } else {
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

    // OSC 7 cwd reporting — bash runs this after each command, emitting
    // `\e]7;file://$HOSTNAME$PWD\a` so the agent can track the shell's cwd.
    // Harmless for /bin/sh (ignored, since PROMPT_COMMAND is a bashism).
    let prompt_command = r#"printf '\033]7;file://%s%s\007' "${HOSTNAME:-}" "$PWD""#;

    // Use pre_exec to redirect stderr to stdout (dup2) so all output goes
    // through a single pipe. This prevents stderr from filling its buffer
    // and deadlocking the child process.
    let mut process = unsafe {
        Command::new(shell_path)
            .env("PROMPT_COMMAND", prompt_command)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .pre_exec(|| {
                // dup2(stdout_fd=1, stderr_fd=2) — stderr now writes to stdout
                if libc::dup2(1, 2) == -1 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            })
            .spawn()?
    };

    // Drop the stderr handle — it's not connected to anything useful anymore
    // since we redirected fd 2 to fd 1 in the child
    drop(process.stderr.take());

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
