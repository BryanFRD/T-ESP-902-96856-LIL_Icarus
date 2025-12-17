use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use http::StatusCode;
use tracing::error;

use crate::api::auth::JwtClaims;
use crate::state::GlobalState;

use super::{CompanyError, CompanyReturn, CreateCompanyPayload, UpdateCompanyPayload};

#[utoipa::path(
    post,
    path = "/v1/teams/{team_id}/company",
    tag = "Teams",
    request_body = CreateCompanyPayload,
    responses(
        (status = 201, description = "Company created", body = CompanyReturn),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Team not found"),
        (status = 409, description = "Company with this SIRET already exists in the team"),
    ),
)]
pub async fn create_company(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path(team_id): Path<String>,
    Json(payload): Json<CreateCompanyPayload>,
) -> impl IntoResponse {
    match super::create_company(
        &state,
        &team_id,
        &jwt_user.sub.to_string(),
        payload,
        jwt_user.is_admin(),
    ) {
        Ok(company) => {
            (StatusCode::CREATED, Json(CompanyReturn::from(company))).into_response()
        }
        Err(CompanyError::NotAuthorized) => {
            (StatusCode::FORBIDDEN, Json("error.user.auth.forbidden")).into_response()
        }
        Err(CompanyError::TeamNotFound) => {
            (StatusCode::NOT_FOUND, Json("error.team.notFound")).into_response()
        }
        Err(CompanyError::DuplicateSiret) => {
            (StatusCode::CONFLICT, Json("error.company.duplicateSiret")).into_response()
        }
        Err(e) => {
            error!("Failed to create company: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("error.server.internalError"),
            )
                .into_response()
        }
    }
}

#[utoipa::path(
    get,
    path = "/v1/teams/{team_id}/company",
    tag = "Teams",
    responses(
        (status = 200, description = "List companies", body = [CompanyReturn]),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - not a team member"),
        (status = 404, description = "Team not found"),
    ),
)]
pub async fn list_companies(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path(team_id): Path<String>,
) -> impl IntoResponse {
    match super::list_companies(&state, &team_id, &jwt_user.sub.to_string()) {
        Ok(companies) => {
            let companies: Vec<CompanyReturn> =
                companies.into_iter().map(CompanyReturn::from).collect();
            Json(companies).into_response()
        }
        Err(CompanyError::TeamNotFound) => {
            (StatusCode::NOT_FOUND, Json("error.team.notFound")).into_response()
        }
        Err(CompanyError::NotTeamMember) => {
            (StatusCode::FORBIDDEN, Json("error.user.auth.forbidden")).into_response()
        }
        Err(e) => {
            error!("Failed to list companies: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("error.server.internalError"),
            )
                .into_response()
        }
    }
}

#[utoipa::path(
    get,
    path = "/v1/teams/{team_id}/company/{company_id}",
    tag = "Teams",
    responses(
        (status = 200, description = "Company found", body = CompanyReturn),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - not a team member"),
        (status = 404, description = "Team or company not found"),
    ),
)]
pub async fn get_company(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path((team_id, company_id)): Path<(String, String)>,
) -> impl IntoResponse {
    match super::get_company(&state, &team_id, &company_id, &jwt_user.sub.to_string()) {
        Ok(company) => Json(CompanyReturn::from(company)).into_response(),
        Err(CompanyError::TeamNotFound) => {
            (StatusCode::NOT_FOUND, Json("error.team.notFound")).into_response()
        }
        Err(CompanyError::NotFound) => {
            (StatusCode::NOT_FOUND, Json("error.company.notFound")).into_response()
        }
        Err(CompanyError::NotTeamMember) => {
            (StatusCode::FORBIDDEN, Json("error.user.auth.forbidden")).into_response()
        }
        Err(e) => {
            error!("Failed to get company: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("error.server.internalError"),
            )
                .into_response()
        }
    }
}

#[utoipa::path(
    patch,
    path = "/v1/teams/{team_id}/company/{company_id}",
    tag = "Teams",
    request_body = UpdateCompanyPayload,
    responses(
        (status = 200, description = "Company updated", body = CompanyReturn),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Team or company not found"),
        (status = 409, description = "Company with this SIRET already exists in the team"),
    ),
)]
pub async fn update_company(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path((team_id, company_id)): Path<(String, String)>,
    Json(payload): Json<UpdateCompanyPayload>,
) -> impl IntoResponse {
    match super::update_company(
        &state,
        &team_id,
        &company_id,
        &jwt_user.sub.to_string(),
        payload,
        jwt_user.is_admin(),
    ) {
        Ok(company) => Json(CompanyReturn::from(company)).into_response(),
        Err(CompanyError::NotAuthorized) => {
            (StatusCode::FORBIDDEN, Json("error.user.auth.forbidden")).into_response()
        }
        Err(CompanyError::TeamNotFound) => {
            (StatusCode::NOT_FOUND, Json("error.team.notFound")).into_response()
        }
        Err(CompanyError::NotFound) => {
            (StatusCode::NOT_FOUND, Json("error.company.notFound")).into_response()
        }
        Err(CompanyError::DuplicateSiret) => {
            (StatusCode::CONFLICT, Json("error.company.duplicateSiret")).into_response()
        }
        Err(e) => {
            error!("Failed to update company: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("error.server.internalError"),
            )
                .into_response()
        }
    }
}

#[utoipa::path(
    delete,
    path = "/v1/teams/{team_id}/company/{company_id}",
    tag = "Teams",
    responses(
        (status = 204, description = "Company deleted"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Team or company not found"),
    ),
)]
pub async fn delete_company(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path((team_id, company_id)): Path<(String, String)>,
) -> impl IntoResponse {
    match super::delete_company(
        &state,
        &team_id,
        &company_id,
        &jwt_user.sub.to_string(),
        jwt_user.is_admin(),
    ) {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(CompanyError::NotAuthorized) => {
            (StatusCode::FORBIDDEN, Json("error.user.auth.forbidden")).into_response()
        }
        Err(CompanyError::TeamNotFound) => {
            (StatusCode::NOT_FOUND, Json("error.team.notFound")).into_response()
        }
        Err(CompanyError::NotFound) => {
            (StatusCode::NOT_FOUND, Json("error.company.notFound")).into_response()
        }
        Err(e) => {
            error!("Failed to delete company: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("error.server.internalError"),
            )
                .into_response()
        }
    }
}
