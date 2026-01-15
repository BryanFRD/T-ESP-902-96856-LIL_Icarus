use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::schema::{audits, teams, users};
use crate::state::GlobalState;
use diesel::prelude::*;

/// Collect and record system/process metrics
///
/// This function reads from `/proc/self/*` to gather process metrics.
/// Falls back silently on non-Linux systems or if files cannot be read.
#[allow(clippy::cast_precision_loss)]
pub fn collect_process_metrics() {
    // Collect start time once (it doesn't change)
    if let Some(start_time) = get_process_start_time() {
        metrics::gauge!("process_start_time_seconds").set(start_time);
    }

    // Collect CPU and memory stats
    if let Some((cpu_time, rss, vms)) = read_proc_stat() {
        metrics::gauge!("process_cpu_seconds_total").set(cpu_time);
        metrics::gauge!("process_resident_memory_bytes").set(rss as f64);
        metrics::gauge!("process_virtual_memory_bytes").set(vms as f64);
    }

    // Collect file descriptor count
    if let Some(fd_count) = count_open_fds() {
        metrics::gauge!("process_open_fds").set(fd_count as f64);
    }
}

/// Collect and record database pool metrics
pub fn collect_database_pool_metrics(state: &GlobalState) {
    let pool_state = state.db_pool.state();
    let active = pool_state.connections - pool_state.idle_connections;

    metrics::gauge!("db_connections_active").set(f64::from(active));
    metrics::gauge!("db_connections_idle").set(f64::from(pool_state.idle_connections));
}

/// Collect and record business metrics (entity counts)
#[allow(clippy::cast_precision_loss)]
pub fn collect_business_metrics(state: &GlobalState) {
    let Ok(mut conn) = state.get_db_conn() else {
        return;
    };

    // Count users
    if let Ok(count) = users::table.count().get_result::<i64>(&mut conn) {
        metrics::gauge!("users_total").set(count as f64);
    }

    // Count teams
    if let Ok(count) = teams::table.count().get_result::<i64>(&mut conn) {
        metrics::gauge!("teams_total").set(count as f64);
    }

    // Count audits (non-archived)
    if let Ok(count) = audits::table
        .filter(audits::is_archived.eq(false))
        .count()
        .get_result::<i64>(&mut conn)
    {
        metrics::gauge!("audits_total", "archived" => "false").set(count as f64);
    }

    // Count audits (archived)
    if let Ok(count) = audits::table
        .filter(audits::is_archived.eq(true))
        .count()
        .get_result::<i64>(&mut conn)
    {
        metrics::gauge!("audits_total", "archived" => "true").set(count as f64);
    }
}

/// Start a background task that periodically collects all metrics
pub fn start_metrics_collector(state: GlobalState) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(15));
        loop {
            interval.tick().await;
            collect_process_metrics();
            collect_database_pool_metrics(&state);
            collect_business_metrics(&state);
        }
    });
    tracing::info!("System metrics collector started (15s interval)");
}

/// Get process start time in seconds since UNIX epoch
#[allow(clippy::cast_precision_loss)]
fn get_process_start_time() -> Option<f64> {
    let stat = fs::read_to_string("/proc/self/stat").ok()?;
    let fields: Vec<&str> = stat.split_whitespace().collect();

    // Field 22 is starttime (in clock ticks since boot)
    let start_ticks: u64 = fields.get(21)?.parse().ok()?;

    // Get system boot time
    let uptime = fs::read_to_string("/proc/uptime").ok()?;
    let uptime_secs: f64 = uptime.split_whitespace().next()?.parse().ok()?;

    // Get current time
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()?
        .as_secs_f64();

    // Calculate process start time
    // Clock ticks per second (typically 100 on Linux)
    let ticks_per_sec = 100.0;
    let start_time_relative = start_ticks as f64 / ticks_per_sec;
    let boot_time = now - uptime_secs;
    let start_time = boot_time + start_time_relative;

    Some(start_time)
}

/// Read CPU time and memory stats from `/proc/self/stat`
///
/// Returns (`cpu_seconds`, `rss_bytes`, `vms_bytes`)
#[allow(clippy::cast_precision_loss)]
fn read_proc_stat() -> Option<(f64, u64, u64)> {
    let stat = fs::read_to_string("/proc/self/stat").ok()?;
    let fields: Vec<&str> = stat.split_whitespace().collect();

    // Fields 14 and 15 are utime and stime (CPU time in clock ticks)
    let utime: u64 = fields.get(13)?.parse().ok()?;
    let stime: u64 = fields.get(14)?.parse().ok()?;

    // Field 24 is RSS (in pages)
    let rss_pages: u64 = fields.get(23)?.parse().ok()?;

    // Field 23 is VMS (virtual memory size in bytes)
    let vms: u64 = fields.get(22)?.parse().ok()?;

    // Convert clock ticks to seconds (typically 100 ticks/sec on Linux)
    let ticks_per_sec = 100.0;
    let cpu_seconds = (utime + stime) as f64 / ticks_per_sec;

    // Convert RSS pages to bytes (typically 4096 bytes/page)
    let page_size = 4096;
    let rss_bytes = rss_pages * page_size;

    Some((cpu_seconds, rss_bytes, vms))
}

/// Count open file descriptors by reading `/proc/self/fd/`
fn count_open_fds() -> Option<usize> {
    let fd_dir = fs::read_dir("/proc/self/fd").ok()?;
    Some(fd_dir.count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "linux")]
    fn test_collect_process_metrics_does_not_panic() {
        // Just ensure it doesn't panic
        collect_process_metrics();
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_read_proc_stat() {
        let result = read_proc_stat();
        // On Linux, this should return Some
        assert!(result.is_some());

        if let Some((cpu, rss, vms)) = result {
            assert!(cpu >= 0.0);
            assert!(rss > 0);
            assert!(vms > 0);
        }
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_count_open_fds() {
        let result = count_open_fds();
        assert!(result.is_some());

        if let Some(count) = result {
            // Should have at least a few FDs open (stdin, stdout, stderr)
            assert!(count >= 3);
        }
    }
}
