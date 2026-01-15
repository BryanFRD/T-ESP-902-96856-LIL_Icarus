use axum::Router;
use metrics_exporter_prometheus::PrometheusHandle;
use std::sync::OnceLock;

use crate::state::GlobalState;

use super::endpoints::metrics_handler;

static PROMETHEUS_HANDLE: OnceLock<PrometheusHandle> = OnceLock::new();

/// Install the Prometheus metrics recorder globally
/// Should be called once during application startup
pub fn install_recorder(state: GlobalState) {
    let builder = metrics_exporter_prometheus::PrometheusBuilder::new();
    let handle = builder
        .set_buckets(&[0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0])
        .expect("Failed to set histogram buckets")
        .install_recorder()
        .expect("Failed to install Prometheus recorder");

    PROMETHEUS_HANDLE
        .set(handle)
        .expect("Prometheus handle already set");

    init_metrics_descriptions();

    // Start background task to collect system metrics
    super::system_metrics::start_metrics_collector(state);
}

/// Initialize metric descriptions for better Prometheus/Grafana documentation
fn init_metrics_descriptions() {
    // HTTP metrics
    metrics::describe_histogram!(
        "http_request_duration_seconds",
        "HTTP request duration in seconds"
    );
    metrics::describe_counter!("http_requests_total", "Total number of HTTP requests");
    metrics::describe_gauge!(
        "http_requests_in_flight",
        "Number of HTTP requests currently being processed"
    );

    // System/process metrics
    metrics::describe_gauge!(
        "process_cpu_seconds_total",
        "Total CPU time consumed by the process in seconds"
    );
    metrics::describe_gauge!(
        "process_resident_memory_bytes",
        "Resident memory size in bytes"
    );
    metrics::describe_gauge!(
        "process_virtual_memory_bytes",
        "Virtual memory size in bytes"
    );
    metrics::describe_gauge!("process_open_fds", "Number of open file descriptors");
    metrics::describe_gauge!(
        "process_start_time_seconds",
        "Start time of the process since unix epoch in seconds"
    );

    // Database pool metrics
    metrics::describe_gauge!("db_connections_active", "Active database connections");
    metrics::describe_gauge!("db_connections_idle", "Idle database connections");
    metrics::describe_counter!("db_queries_total", "Total database queries executed");

    // Auth metrics
    metrics::describe_counter!("auth_login_total", "Total login attempts");
    metrics::describe_counter!("auth_register_total", "Total registration attempts");

    // Business metrics
    metrics::describe_gauge!("users_total", "Total number of users");
    metrics::describe_gauge!("teams_total", "Total number of teams");
    metrics::describe_gauge!("audits_total", "Total number of audits");

    // HTTP error metrics
    metrics::describe_counter!("http_errors_total", "Total HTTP errors (4xx/5xx)");
}

/// Get the Prometheus handle for rendering metrics
pub(crate) fn get_handle() -> &'static PrometheusHandle {
    PROMETHEUS_HANDLE
        .get()
        .expect("Prometheus recorder not installed")
}

pub fn get_routes() -> Router {
    Router::new().route("/metrics", axum::routing::get(metrics_handler))
}
