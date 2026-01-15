use axum::response::IntoResponse;

/// Prometheus metrics endpoint
/// Returns metrics in JSON format
pub async fn metrics_handler() -> impl IntoResponse {
    let handle = super::services::get_handle();
    handle.render()
}
