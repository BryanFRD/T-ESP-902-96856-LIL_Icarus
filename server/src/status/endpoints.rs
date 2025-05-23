use axum::{Json, extract::State, response::IntoResponse};
use http::StatusCode;

use crate::state::GlobalState;

pub async fn get_status(State(state): State<GlobalState>) -> impl IntoResponse {
    let secrets = state.get_secrets();
    let status = secrets.status.clone();
    let status_message = secrets.status_message.clone();

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": status,
            "message": status_message,
        })),
    )
        .into_response()
}
