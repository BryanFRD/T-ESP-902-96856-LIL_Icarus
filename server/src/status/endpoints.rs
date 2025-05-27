use axum::{Json, extract::State, response::IntoResponse};
use http::StatusCode;

use crate::state::GlobalState;

use super::models::Status;

#[utoipa::path(
    get,
    path = "/v1/status",
    description = "Get the current status of the application",
    responses(
        (status = 200, description = "Status retrieved successfully", body = Status),
    ),
    tag = "Status"
)]
pub async fn get_status(State(state): State<GlobalState>) -> impl IntoResponse {
    let secrets = state.get_secrets();
    let status = secrets.status.clone();
    let status_message = secrets.status_message.clone();

    (StatusCode::OK, Json(Status::new(status, status_message))).into_response()
}
