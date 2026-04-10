use axum::{Json, extract::State, response::IntoResponse};
use http::StatusCode;

use crate::state::GlobalState;

use super::models::DashboardMetrics;
use super::services::collect_metrics;

#[utoipa::path(
    get,
    path = "/v1/dashboard",
    description = "Get dashboard metrics (admin only)",
    responses(
        (status = 200, description = "Dashboard metrics retrieved successfully", body = DashboardMetrics),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - Admin only"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Dashboard",
    security(("cookieAuth" = []))
)]
pub async fn get_dashboard(State(state): State<GlobalState>) -> impl IntoResponse {
    match collect_metrics(&state) {
        Ok(metrics) => (StatusCode::OK, Json(metrics)).into_response(),
        Err(e) => {
            tracing::error!("Failed to collect dashboard metrics: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to collect metrics"})),
            )
                .into_response()
        }
    }
}
