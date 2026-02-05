use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use http::StatusCode;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::api::auth::models::JwtClaims;
use crate::api::events::{EventFilters, EventResponse};
use crate::helpers::{internal_error, not_found_error};
use crate::state::GlobalState;

#[utoipa::path(
    get,
    path = "/v1/events",
    tag = "Events",
    params(
        ("user_id" = Option<String>, Query, description = "Filter by user ID"),
        ("resource_type" = Option<String>, Query, description = "Filter by resource type"),
        ("resource_id" = Option<String>, Query, description = "Filter by resource ID"),
        ("action" = Option<String>, Query, description = "Filter by action"),
        ("date_from" = Option<String>, Query, description = "Filter from date (ISO 8601)"),
        ("date_to" = Option<String>, Query, description = "Filter to date (ISO 8601)"),
        ("limit" = Option<i64>, Query, description = "Limit results (default: 100, max: 1000)"),
        ("offset" = Option<i64>, Query, description = "Offset for pagination"),
        ("anonymized" = Option<bool>, Query, description = "Anonymize sensitive data (default: true, admin can set to false)"),
    ),
    responses(
        (status = 200, description = "Events retrieved successfully", body = Vec<EventResponse>),
        (status = 401, description = "Unauthorized"),
    ),
)]
pub async fn list_events(
    jwt: JwtClaims,
    State(state): State<GlobalState>,
    Query(filters): Query<EventFilters>,
) -> impl IntoResponse {
    let is_admin = jwt.is_admin();
    let should_redact = filters.anonymized || !is_admin;

    match super::query_events(&state, filters) {
        Ok(events) => {
            let responses: Vec<EventResponse> = events
                .into_iter()
                .map(|event| super::decrypt_and_redact_event(&state, event, !should_redact))
                .map(EventResponse::from)
                .collect();
            (StatusCode::OK, Json(responses)).into_response()
        }
        Err(e) => internal_error("Failed to query events", e),
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AnonymizedQuery {
    #[serde(default = "default_anonymized_query")]
    pub anonymized: bool,
}

fn default_anonymized_query() -> bool {
    true
}

#[utoipa::path(
    get,
    path = "/v1/events/{id}",
    tag = "Events",
    params(
        ("anonymized" = Option<bool>, Query, description = "Anonymize sensitive data (default: true, admin can set to false)"),
    ),
    responses(
        (status = 200, description = "Event retrieved successfully", body = EventResponse),
        (status = 404, description = "Event not found"),
        (status = 401, description = "Unauthorized"),
    ),
)]
pub async fn get_event(
    jwt: JwtClaims,
    State(state): State<GlobalState>,
    Path(id): Path<String>,
    Query(query): Query<AnonymizedQuery>,
) -> impl IntoResponse {
    let is_admin = jwt.is_admin();
    let should_redact = query.anonymized || !is_admin;

    match super::get_event_by_id(&state, &id) {
        Ok(Some(event)) => {
            let redacted = super::decrypt_and_redact_event(&state, event, !should_redact);
            (StatusCode::OK, Json(EventResponse::from(redacted))).into_response()
        }
        Ok(None) => not_found_error("error.event.notFound"),
        Err(e) => internal_error("Failed to get event", e),
    }
}
