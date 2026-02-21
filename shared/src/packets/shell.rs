use crate::Packet;
use bincode::{Decode, Encode};

/// Request to spawn a reverse shell on the agent
#[derive(Debug, Clone, Encode, Decode, Packet)]
#[packet(code = 0x0C)]
pub struct ShellRequest {
    pub shell_type: ShellType,
}

impl ShellRequest {
    #[must_use]
    pub fn new(shell_type: ShellType) -> Self {
        Self { shell_type }
    }

    /// Request automatic shell detection
    #[must_use]
    pub fn auto() -> Self {
        Self::new(ShellType::Auto)
    }

    /// Request bash shell (Unix)
    #[must_use]
    pub fn bash() -> Self {
        Self::new(ShellType::Bash)
    }

    /// Request sh shell (Unix)
    #[must_use]
    pub fn sh() -> Self {
        Self::new(ShellType::Sh)
    }

    /// Request cmd.exe (Windows)
    #[must_use]
    pub fn cmd() -> Self {
        Self::new(ShellType::Cmd)
    }

    /// Request `PowerShell` (Windows)
    #[must_use]
    pub fn powershell() -> Self {
        Self::new(ShellType::PowerShell)
    }
}

/// Type of shell to spawn
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub enum ShellType {
    /// Automatically detect best shell for platform
    Auto,
    /// Bash shell (Unix only)
    Bash,
    /// Bourne shell (Unix only)
    Sh,
    /// Windows Command Prompt
    Cmd,
    /// `PowerShell`
    PowerShell,
}
