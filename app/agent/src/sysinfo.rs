use shared::packets::SystemInfo;
use sysinfo::System;

/// Collects current system information.
#[must_use]
pub fn collect_system_info() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_name = sys
        .cpus()
        .first()
        .map_or_else(|| "Unknown".to_string(), |cpu| cpu.brand().to_string());

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
    }
}
