use crate::Packet;
use bincode::{Decode, Encode};

/// Request to spawn a reverse shell on the agent
#[derive(Debug, Clone, Encode, Decode, Packet)]
#[packet(code = 0x0C)]
pub struct ShellRequest {
    pub shell_type: ShellType,
}

/// Shell current working directory update, emitted by the agent whenever
/// the spawned shell reports a new `$PWD` via the OSC 7 escape sequence.
#[derive(Debug, Clone, Encode, Decode, Packet)]
#[packet(code = 0x0F)]
pub struct ShellCwd {
    pub stream_id: u32,
    pub path: String,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packets::Packet;

    #[test]
    fn factory_methods() {
        assert_eq!(ShellRequest::auto().shell_type, ShellType::Auto);
        assert_eq!(ShellRequest::bash().shell_type, ShellType::Bash);
        assert_eq!(ShellRequest::sh().shell_type, ShellType::Sh);
        assert_eq!(ShellRequest::cmd().shell_type, ShellType::Cmd);
        assert_eq!(ShellRequest::powershell().shell_type, ShellType::PowerShell);
    }

    #[test]
    fn serialize_deserialize_roundtrip() {
        let req = ShellRequest::bash();
        let bytes = req.serialize().unwrap();
        let decoded = ShellRequest::deserialize(&bytes[1..]).unwrap();
        assert_eq!(decoded.shell_type, ShellType::Bash);
    }

    #[test]
    fn all_shell_types_roundtrip() {
        let types = [ShellType::Auto, ShellType::Bash, ShellType::Sh, ShellType::Cmd, ShellType::PowerShell];
        for st in types {
            let req = ShellRequest::new(st);
            let bytes = req.serialize().unwrap();
            let decoded = ShellRequest::deserialize(&bytes[1..]).unwrap();
            assert_eq!(decoded.shell_type, st);
        }
    }

    #[test]
    fn packet_code_is_correct() {
        assert_eq!(ShellRequest::packet_code(), 0x0C);
    }
}
