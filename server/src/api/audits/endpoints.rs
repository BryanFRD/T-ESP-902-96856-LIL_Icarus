use std::fmt::Write as _;

use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use http::StatusCode;
use ulid::Ulid;
use validator::Validate;

use crate::{
    api::auth::JwtClaims,
    api::events::AuditLogFilters,
    helpers::{bad_request_error, internal_error, not_found_error},
    state::GlobalState,
};

use super::{AuditError, AuditResponse, CreateAuditPayload, UpdateAuditPayload};

#[utoipa::path(
    get,
    path = "/v1/audits/me",
    tag = "Audits",
    responses(
        (status = 200, description = "Audits retrieved successfully", body = Vec<AuditResponse>),
        (status = 401, description = "Unauthorized"),
    ),
    security(
        ("cookieAuth" = [])
    ),
)]
pub async fn list_my_audits(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
) -> impl IntoResponse {
    match super::list_user_audits(&state, &jwt_user.sub.to_string(), &jwt_user.role) {
        Ok(audits) => (StatusCode::OK, Json(audits)).into_response(),
        Err(e) => internal_error("Failed to list audits", e),
    }
}

#[utoipa::path(
    post,
    path = "/v1/audits/me",
    tag = "Audits",
    request_body = CreateAuditPayload,
    responses(
        (status = 201, description = "Audit created successfully", body = AuditResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - must be team owner or admin"),
        (status = 404, description = "Team or company not found"),
    ),
    security(
        ("cookieAuth" = [])
    ),
)]
pub async fn create_audit(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Json(payload): Json<CreateAuditPayload>,
) -> impl IntoResponse {
    if let Err(e) = payload.validate() {
        return bad_request_error(&format!("Validation error: {e}"));
    }

    match super::create_audit(
        &state,
        &jwt_user.sub.to_string(),
        payload,
        jwt_user.is_admin(),
    ) {
        Ok(audit) => (StatusCode::CREATED, Json(audit)).into_response(),
        Err(AuditError::TeamNotFound) => not_found_error("error.audit.teamNotFound"),
        Err(AuditError::CompanyNotFound) => not_found_error("error.audit.companyNotFound"),
        Err(AuditError::CompanyTeamMismatch) => {
            bad_request_error("error.audit.companyTeamMismatch")
        }
        Err(AuditError::MustBeOwnerOrAdmin) => (
            StatusCode::FORBIDDEN,
            Json("error.audit.mustBeOwnerOrAdmin"),
        )
            .into_response(),
        Err(AuditError::EndDateInPast) => bad_request_error("error.audit.endDateInPast"),
        Err(e) => internal_error("Failed to create audit", e),
    }
}

#[utoipa::path(
    patch,
    path = "/v1/audits/{audit_id}",
    tag = "Audits",
    request_body = UpdateAuditPayload,
    responses(
        (status = 200, description = "Audit updated successfully", body = AuditResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - audit archived or insufficient permissions"),
        (status = 404, description = "Audit not found"),
    ),
    security(
        ("cookieAuth" = [])
    ),
)]
pub async fn update_audit(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path(audit_id): Path<Ulid>,
    Json(payload): Json<UpdateAuditPayload>,
) -> impl IntoResponse {
    if let Err(e) = payload.validate() {
        return bad_request_error(&format!("Validation error: {e}"));
    }

    if payload.name.is_none()
        && payload.description.is_none()
        && payload.end_date.is_none()
        && !payload.clear_end_date
    {
        return bad_request_error("error.audit.update.noFieldProvided");
    }

    match super::update_audit(
        &state,
        &audit_id.to_string(),
        &jwt_user.sub.to_string(),
        &payload,
        jwt_user.is_admin(),
    ) {
        Ok(audit) => (StatusCode::OK, Json(audit)).into_response(),
        Err(AuditError::NotFound) => not_found_error("error.audit.notFound"),
        Err(AuditError::AuditArchived) => {
            (StatusCode::FORBIDDEN, Json("error.audit.archived")).into_response()
        }
        Err(AuditError::MustBeOwnerOrAdmin) => (
            StatusCode::FORBIDDEN,
            Json("error.audit.mustBeOwnerOrAdmin"),
        )
            .into_response(),
        Err(AuditError::EndDateInPast) => bad_request_error("error.audit.endDateInPast"),
        Err(e) => internal_error("Failed to update audit", e),
    }
}

#[utoipa::path(
    delete,
    path = "/v1/audits/{audit_id}",
    tag = "Audits",
    responses(
        (status = 204, description = "Audit archived successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - audit already archived or insufficient permissions"),
        (status = 404, description = "Audit not found"),
    ),
    security(
        ("cookieAuth" = [])
    ),
)]
pub async fn archive_audit(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path(audit_id): Path<Ulid>,
) -> impl IntoResponse {
    match super::archive_audit(
        &state,
        &audit_id.to_string(),
        &jwt_user.sub.to_string(),
        jwt_user.is_admin(),
    ) {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(AuditError::NotFound) => not_found_error("error.audit.notFound"),
        Err(AuditError::AuditArchived) => {
            (StatusCode::FORBIDDEN, Json("error.audit.alreadyArchived")).into_response()
        }
        Err(AuditError::MustBeOwnerOrAdmin) => (
            StatusCode::FORBIDDEN,
            Json("error.audit.mustBeOwnerOrAdmin"),
        )
            .into_response(),
        Err(e) => internal_error("Failed to archive audit", e),
    }
}

#[utoipa::path(
    delete,
    path = "/v1/audits/{audit_id}/clients/{user_id}",
    tag = "Audits",
    responses(
        (status = 204, description = "Client removed successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - audit archived or insufficient permissions"),
        (status = 404, description = "Audit or client not found"),
    ),
    security(
        ("cookieAuth" = [])
    ),
)]
pub async fn remove_client(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path((audit_id, user_id)): Path<(Ulid, Ulid)>,
) -> impl IntoResponse {
    match super::remove_client_from_audit(
        &state,
        &audit_id.to_string(),
        &user_id.to_string(),
        &jwt_user.sub.to_string(),
        jwt_user.is_admin(),
    ) {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(AuditError::NotFound) => not_found_error("error.audit.notFound"),
        Err(AuditError::ClientNotInAudit) => not_found_error("error.audit.clientNotInAudit"),
        Err(AuditError::AuditArchived) => {
            (StatusCode::FORBIDDEN, Json("error.audit.archived")).into_response()
        }
        Err(AuditError::MustBeOwnerOrAdmin) => (
            StatusCode::FORBIDDEN,
            Json("error.audit.mustBeOwnerOrAdmin"),
        )
            .into_response(),
        Err(e) => internal_error("Failed to remove client", e),
    }
}

#[utoipa::path(
    get,
    path = "/v1/audits/{audit_id}/logs",
    tag = "Audits",
    params(
        ("audit_id" = String, Path, description = "Audit ID"),
        ("includeUser" = Option<bool>, Query, description = "Include user details in response"),
        ("action" = Option<String>, Query, description = "Filter by event action"),
        ("limit" = Option<i64>, Query, description = "Maximum number of results (default 50, max 200)"),
        ("offset" = Option<i64>, Query, description = "Number of results to skip"),
    ),
    responses(
        (status = 200, description = "Audit logs retrieved successfully", body = crate::api::events::PaginatedEventLogResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - not a team member"),
        (status = 404, description = "Audit not found"),
    ),
    security(
        ("cookieAuth" = [])
    ),
)]
pub async fn list_audit_logs(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path(audit_id): Path<Ulid>,
    Query(filters): Query<AuditLogFilters>,
) -> impl IntoResponse {
    let audit_id_str = audit_id.to_string();

    let audit = match super::get_audit(&state, &audit_id_str) {
        Ok(a) => a,
        Err(AuditError::NotFound) => return not_found_error("error.audit.notFound"),
        Err(e) => return internal_error("Failed to get audit", e),
    };

    let user_id_str = jwt_user.sub.to_string();
    if !jwt_user.is_admin()
        && crate::api::teams::get_team_member(&state, &audit.team_id, &user_id_str)
            .ok()
            .flatten()
            .is_none()
    {
        return (StatusCode::FORBIDDEN, Json("error.audit.mustBeTeamMember")).into_response();
    }

    match crate::api::events::query_audit_logs(&state, &audit_id_str, &filters) {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => internal_error("Failed to query audit logs", e),
    }
}

#[utoipa::path(
    get,
    path = "/v1/audits/{audit_id}/logs/export",
    tag = "Audits",
    params(
        ("audit_id" = String, Path, description = "Audit ID"),
    ),
    responses(
        (status = 200, description = "Audit events exported as CSV", content_type = "text/csv"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Audit not found"),
    ),
    security(
        ("cookieAuth" = [])
    ),
)]
pub async fn export_audit_logs(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path(audit_id): Path<Ulid>,
) -> impl IntoResponse {
    let audit_id_str = audit_id.to_string();

    let audit = match super::get_audit(&state, &audit_id_str) {
        Ok(a) => a,
        Err(AuditError::NotFound) => return not_found_error("error.audit.notFound"),
        Err(e) => return internal_error("Failed to get audit", e),
    };

    let user_id_str = jwt_user.sub.to_string();
    if !jwt_user.is_admin()
        && crate::api::teams::get_team_member(&state, &audit.team_id, &user_id_str)
            .ok()
            .flatten()
            .is_none()
    {
        return (StatusCode::FORBIDDEN, Json("error.audit.mustBeTeamMember")).into_response();
    }

    let filters = AuditLogFilters {
        include_user: Some(true),
        action: None,
        limit: Some(200),
        offset: Some(0),
    };

    let mut all_rows = Vec::new();
    let mut offset: i64 = 0;
    loop {
        let page_filters = AuditLogFilters {
            offset: Some(offset),
            ..filters.clone()
        };
        match crate::api::events::query_audit_logs(&state, &audit_id_str, &page_filters) {
            Ok(page) => {
                let fetched = i64::try_from(page.data.len()).unwrap_or(i64::MAX);
                all_rows.extend(page.data);
                let total_len = i64::try_from(all_rows.len()).unwrap_or(i64::MAX);
                if fetched < page.limit || total_len >= page.total {
                    break;
                }
                offset += fetched;
            }
            Err(e) => return internal_error("Failed to query audit logs", e),
        }
    }

    let mut csv = String::from("created_at,action,user,method,path,resource_type,resource_id\n");
    for row in all_rows {
        let user_display = row
            .user
            .as_ref()
            .map(|u| format!("{} <{}>", u.username, u.email))
            .or(row.user_id.clone())
            .unwrap_or_default();
        let _ = writeln!(
            csv,
            "{},{},{},{},{},{},{}",
            csv_escape(&row.created_at),
            csv_escape(&format!("{:?}", row.action)),
            csv_escape(&user_display),
            csv_escape(&row.request_method),
            csv_escape(&row.request_path),
            csv_escape(&format!("{:?}", row.resource_type)),
            csv_escape(row.resource_id.as_deref().unwrap_or("")),
        );
    }

    let filename = format!(
        "audit-events-{}.csv",
        audit.name.replace(' ', "-").to_lowercase()
    );
    let headers = [
        (http::header::CONTENT_TYPE, "text/csv".to_string()),
        (
            http::header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{filename}\""),
        ),
    ];
    (StatusCode::OK, headers, csv).into_response()
}

fn csv_escape(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}
