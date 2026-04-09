use shared::packets::SystemInfo;
use std::process::Command;
use sysinfo::{Disks, System};

/// Detects GPU name using platform-specific methods.
fn detect_gpu() -> Option<String> {
    if let Ok(output) = Command::new("nvidia-smi")
        .args(["--query-gpu=name", "--format=csv,noheader"])
        .output()
        && output.status.success()
    {
        let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !name.is_empty() {
            return Some(name);
        }
    }

    #[cfg(target_os = "windows")]
    if let Ok(output) = Command::new("wmic")
        .args(["path", "win32_VideoController", "get", "name"])
        .output()
        && output.status.success()
    {
        let text = String::from_utf8_lossy(&output.stdout);
        if let Some(name) = text.lines().skip(1).map(str::trim).find(|l| !l.is_empty()) {
            return Some(name.to_string());
        }
    }

    #[cfg(target_os = "linux")]
    if let Ok(output) = Command::new("lspci").output()
        && output.status.success()
    {
        let text = String::from_utf8_lossy(&output.stdout);
        if let Some(line) = text
            .lines()
            .find(|l| l.contains("VGA") || l.contains("3D controller"))
            && let Some(name) = line.split(": ").nth(1)
        {
            return Some(name.to_string());
        }
    }

    None
}

/// Gets the current username.
fn get_username() -> String {
    #[cfg(target_os = "linux")]
    {
        std::env::var("USER").unwrap_or_else(|_| {
            Command::new("whoami")
                .output()
                .ok()
                .map_or_else(|| "Unknown".to_string(), |o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        })
    }
    #[cfg(target_os = "windows")]
    {
        std::env::var("USERNAME").unwrap_or_else(|_| {
            Command::new("whoami")
                .output()
                .ok()
                .map_or_else(|| "Unknown".to_string(), |o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        })
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        "Unknown".to_string()
    }
}

/// Checks if the current process is running with elevated privileges.
fn is_elevated() -> bool {
    #[cfg(target_os = "linux")]
    {
        // uid 0 = root
        unsafe { libc::geteuid() == 0 }
    }
    #[cfg(target_os = "windows")]
    {
        Command::new("net")
            .args(["session"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        false
    }
}

/// Gets the system locale.
fn get_locale() -> Option<String> {
    #[cfg(target_os = "linux")]
    {
        std::env::var("LANG").ok().or_else(|| std::env::var("LC_ALL").ok())
    }
    #[cfg(target_os = "windows")]
    {
        Command::new("powershell")
            .args(["-NoProfile", "-Command", "(Get-Culture).Name"])
            .output()
            .ok()
            .and_then(|o| {
                if o.status.success() {
                    let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
                    if s.is_empty() { None } else { Some(s) }
                } else {
                    None
                }
            })
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        None
    }
}

/// Gets local IP addresses.
fn get_local_ips() -> Vec<String> {
    #[cfg(target_os = "linux")]
    {
        Command::new("hostname")
            .arg("-I")
            .output()
            .ok()
            .map(|o| {
                String::from_utf8_lossy(&o.stdout)
                    .split_whitespace()
                    .map(String::from)
                    .collect()
            })
            .unwrap_or_default()
    }
    #[cfg(target_os = "windows")]
    {
        Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                "(Get-NetIPAddress -AddressFamily IPv4 | Where-Object { $_.IPAddress -ne '127.0.0.1' }).IPAddress",
            ])
            .output()
            .ok()
            .map(|o| {
                String::from_utf8_lossy(&o.stdout)
                    .lines()
                    .map(|l| l.trim().to_string())
                    .filter(|l| !l.is_empty())
                    .collect()
            })
            .unwrap_or_default()
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        Vec::new()
    }
}

/// Gets the primary MAC address.
fn get_mac_address() -> Option<String> {
    #[cfg(target_os = "linux")]
    {
        // Read from /sys/class/net, skip lo
        if let Ok(entries) = std::fs::read_dir("/sys/class/net") {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name == "lo" {
                    continue;
                }
                let path = entry.path().join("address");
                if let Ok(mac) = std::fs::read_to_string(&path) {
                    let mac = mac.trim().to_uppercase();
                    if !mac.is_empty() && mac != "00:00:00:00:00:00" {
                        return Some(mac);
                    }
                }
            }
        }
        None
    }
    #[cfg(target_os = "windows")]
    {
        Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                "(Get-NetAdapter | Where-Object { $_.Status -eq 'Up' } | Select-Object -First 1).MacAddress",
            ])
            .output()
            .ok()
            .and_then(|o| {
                if o.status.success() {
                    let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
                    if s.is_empty() { None } else { Some(s) }
                } else {
                    None
                }
            })
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        None
    }
}

/// Gets the domain name (Windows AD or Linux domain).
fn get_domain() -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        std::env::var("USERDOMAIN").ok().and_then(|d| {
            // USERDOMAIN equals COMPUTERNAME when not domain-joined
            let computer = std::env::var("COMPUTERNAME").unwrap_or_default();
            if d.eq_ignore_ascii_case(&computer) {
                None
            } else {
                Some(d)
            }
        })
    }
    #[cfg(target_os = "linux")]
    {
        Command::new("hostname")
            .arg("-d")
            .output()
            .ok()
            .and_then(|o| {
                if o.status.success() {
                    let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
                    if s.is_empty() || s == "(none)" {
                        None
                    } else {
                        Some(s)
                    }
                } else {
                    None
                }
            })
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        None
    }
}

/// Gets total and used disk space (only real filesystems, no tmpfs/overlay duplicates).
fn get_disk_usage() -> (u64, u64) {
    let disks = Disks::new_with_refreshed_list();
    let mut total: u64 = 0;
    let mut available: u64 = 0;
    let mut seen_devices = std::collections::HashSet::new();

    for disk in disks.list() {
        let fs = disk.file_system().to_string_lossy().to_lowercase();
        // Skip virtual/pseudo filesystems
        if fs == "tmpfs" || fs == "devtmpfs" || fs == "sysfs" || fs == "proc" || fs == "none" {
            continue;
        }
        // Deduplicate by device name to avoid counting the same device mounted multiple times
        let device = disk.name().to_string_lossy().to_string();
        if !seen_devices.insert(device) {
            continue;
        }
        total = total.saturating_add(disk.total_space());
        available = available.saturating_add(disk.available_space());
    }
    (total, total.saturating_sub(available))
}

/// Known antivirus/EDR process names.
const AV_PROCESSES: &[(&str, &str)] = &[
    ("MsMpEng.exe", "Windows Defender"),
    ("msmpeng", "Windows Defender"),
    ("SecurityHealthService.exe", "Windows Security"),
    ("CSFalconService.exe", "CrowdStrike Falcon"),
    ("falcon-sensor", "CrowdStrike Falcon"),
    ("cb", "Carbon Black"),
    ("CbDefense", "Carbon Black"),
    ("SentinelAgent.exe", "SentinelOne"),
    ("sentineld", "SentinelOne"),
    ("bdagent.exe", "Bitdefender"),
    ("avp.exe", "Kaspersky"),
    ("avgnt.exe", "Avira"),
    ("clamd", "ClamAV"),
    ("freshclam", "ClamAV"),
    ("sophos", "Sophos"),
    ("savd", "Sophos"),
    ("mcshield.exe", "McAfee"),
    ("mfemms.exe", "McAfee"),
    ("NortonSecurity.exe", "Norton"),
    ("esets_daemon", "ESET"),
    ("ekrn.exe", "ESET"),
    ("cylancesvc.exe", "Cylance"),
    ("xagt.exe", "FireEye"),
    ("taniumclient.exe", "Tanium"),
    ("TaniumClient", "Tanium"),
    ("osqueryd", "osquery"),
    ("wazuh-agent", "Wazuh"),
    ("ossec", "Wazuh/OSSEC"),
];

/// Detects running antivirus/EDR products by scanning process names.
fn detect_antivirus() -> Vec<String> {
    let sys = System::new_all();
    let mut found = std::collections::HashSet::new();

    for process in sys.processes().values() {
        let pname = process.name().to_string_lossy().to_lowercase();
        for &(pattern, label) in AV_PROCESSES {
            if pname.contains(&pattern.to_lowercase()) {
                found.insert(label.to_string());
            }
        }
    }

    let mut result: Vec<String> = found.into_iter().collect();
    result.sort();
    result
}

/// Checks if the firewall is enabled.
fn detect_firewall() -> Option<bool> {
    #[cfg(target_os = "windows")]
    {
        Command::new("netsh")
            .args(["advfirewall", "show", "allprofiles", "state"])
            .output()
            .ok()
            .map(|o| {
                let text = String::from_utf8_lossy(&o.stdout).to_lowercase();
                text.contains("on")
            })
    }
    #[cfg(target_os = "linux")]
    {
        // Check iptables rules count
        if let Ok(output) = Command::new("iptables").args(["-L", "-n"]).output()
            && output.status.success()
        {
            let text = String::from_utf8_lossy(&output.stdout);
            let rule_lines = text.lines().filter(|l| !l.starts_with("Chain") && !l.starts_with("target") && !l.trim().is_empty()).count();
            return Some(rule_lines > 0);
        }
        // Check nftables
        if let Ok(output) = Command::new("nft").args(["list", "ruleset"]).output()
            && output.status.success()
        {
            let text = String::from_utf8_lossy(&output.stdout);
            return Some(!text.trim().is_empty());
        }
        // Check ufw
        if let Ok(output) = Command::new("ufw").arg("status").output()
            && output.status.success()
        {
            let text = String::from_utf8_lossy(&output.stdout).to_lowercase();
            return Some(text.contains("active"));
        }
        None
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        None
    }
}

/// Collects current system information.
#[must_use]
pub fn collect_system_info() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_name = sys
        .cpus()
        .first()
        .map_or_else(|| "Unknown".to_string(), |cpu| cpu.brand().to_string());

    let (total_disk_bytes, used_disk_bytes) = get_disk_usage();

    SystemInfo {
        hostname: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
        os_name: System::name().unwrap_or_else(|| "Unknown".to_string()),
        os_version: System::os_version().unwrap_or_else(|| "Unknown".to_string()),
        cpu_name,
        #[allow(clippy::cast_possible_truncation)]
        cpu_count: sys.cpus().len() as u32,
        total_memory_bytes: sys.total_memory(),
        used_memory_bytes: sys.used_memory(),
        uptime_secs: System::uptime(),
        agent_version: env!("CARGO_PKG_VERSION").to_string(),
        gpu_name: detect_gpu(),
        username: get_username(),
        is_elevated: is_elevated(),
        arch: std::env::consts::ARCH.to_string(),
        locale: get_locale(),
        local_ips: get_local_ips(),
        mac_address: get_mac_address(),
        domain: get_domain(),
        total_disk_bytes,
        used_disk_bytes,
        antivirus: detect_antivirus(),
        firewall_enabled: detect_firewall(),
    }
}
