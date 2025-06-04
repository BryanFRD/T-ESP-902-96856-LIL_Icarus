use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use http::StatusCode;
use tracing::error;
use ulid::Ulid;

use crate::{
    auth::JwtClaims,
    state::GlobalState,
    teams::{
        TeamReturn, UpdateTeam,
        models::{CreateTeam, Team},
    },
};

#[utoipa::path(
    post,
    path = "/v1/teams/me",
    tag = "Teams",
    request_body = CreateTeam,
    responses(
        (status = 201, description = "Team created successfully", body = Team),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
pub async fn create_team(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Json(payload): Json<CreateTeam>,
) -> impl IntoResponse {
    match super::create_team(
        &state,
        &jwt_user.sub.to_string(),
        payload.name,
        payload.description,
    ) {
        Ok(team) => (StatusCode::CREATED, Json(team)).into_response(),
        Err(e) => {
            error!("Failed to create team: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}

#[utoipa::path(
    get,
    path = "/v1/teams/me",
    tag = "Teams",
    responses(
        (status = 200, description = "Teams retrieved successfully", body = Vec<TeamReturn>),
        (status = 401, description = "Unauthorized"),
    ),
)]
pub async fn get_my_teams(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
) -> impl IntoResponse {
    match super::get_user_teams(&state, &jwt_user.sub.to_string()) {
        Ok(teams) => (StatusCode::OK, Json(teams)).into_response(),
        Err(e) => {
            error!("Failed to get teams: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}

#[utoipa::path(
    patch,
    path = "/v1/teams/{team_id}",
    tag = "Teams",
    request_body = UpdateTeam,
    responses(
        (status = 200, description = "Updated Team successfully"),
        (status = 404, description = "Team not found"),
        (status = 401, description = "Unauthorized"),
    ),
)]
pub async fn update_team(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path(team_id): Path<Ulid>,
    Json(payload): Json<UpdateTeam>,
) -> impl IntoResponse {
    if payload.name.is_none() && payload.description.is_none() {
        return (StatusCode::BAD_REQUEST, Json("No fields to update")).into_response();
    }

    match super::does_team_exist(&state, &team_id.to_string()) {
        Ok(true) => (),
        Ok(false) => return (StatusCode::NOT_FOUND, Json("Team not found")).into_response(),
        Err(e) => {
            error!("Failed to check team existence: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    match super::is_team_owner(&state, &team_id.to_string(), &jwt_user.sub.to_string()) {
        Ok(true) => (),
        Ok(false) => {
            if !jwt_user.is_admin() {
                return (StatusCode::FORBIDDEN,).into_response();
            }
        }
        Err(e) => {
            error!("Failed to check team ownership: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    match super::update_team(&state, &team_id.to_string(), payload) {
        Ok(_) => (StatusCode::OK).into_response(),
        Err(e) => {
            error!("Failed to update team: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}

#[utoipa::path(
    delete,
    path = "/v1/teams/{team_id}",
    tag = "Teams",
    responses(
        (status = 204, description = "Team deleted successfully"),
        (status = 404, description = "Team not found"),
        (status = 403, description = "Forbidden"),
    ),
)]
pub async fn delete_team(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path(team_id): Path<Ulid>,
) -> impl IntoResponse {
    match super::does_team_exist(&state, &team_id.to_string()) {
        Ok(true) => (),
        Ok(false) => return (StatusCode::NOT_FOUND).into_response(),
        Err(e) => {
            error!("Failed to check team existence: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    match super::is_team_owner(&state, &team_id.to_string(), &jwt_user.sub.to_string()) {
        Ok(true) => (),
        Ok(false) => {
            if !jwt_user.is_admin() {
                return (StatusCode::FORBIDDEN,).into_response();
            }
        }
        Err(e) => {
            error!("Failed to check team ownership: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    };

    match super::delete_team(&state, &team_id.to_string()) {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(e) => {
            error!("Failed to delete team: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}
