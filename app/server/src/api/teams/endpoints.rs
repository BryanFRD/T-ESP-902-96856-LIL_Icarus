use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use http::StatusCode;
use ulid::Ulid;

use crate::api::{
    auth::JwtClaims,
    teams::{
        TeamReturn, TeamRole, UpdateTeam,
        models::{CreateTeam, Team, UpdateMemberRolePayload},
    },
};
use crate::helpers::{bad_request_error, internal_error, not_found_error};
use crate::state::GlobalState;

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
        Err(e) => internal_error("Failed to create team", e),
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
        Err(e) => internal_error("Failed to get teams", e),
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
        return bad_request_error("error.team.update.validate.noFieldProvided");
    }

    match super::does_team_exist(&state, &team_id.to_string()) {
        Ok(true) => (),
        Ok(false) => return not_found_error("error.team.notFound"),
        Err(e) => {
            return internal_error("Failed to check team existence", e);
        }
    }

    match super::is_team_owner(&state, &team_id.to_string(), &jwt_user.sub.to_string()) {
        Ok(true) => (),
        Ok(false) => {
            if !jwt_user.is_admin() {
                return (StatusCode::FORBIDDEN, Json("error.user.auth.forbidden")).into_response();
            }
        }
        Err(e) => {
            return internal_error("Failed to check team ownership", e);
        }
    }

    match super::update_team(&state, &team_id.to_string(), &payload) {
        Ok(_) => (StatusCode::OK).into_response(),
        Err(e) => internal_error("Failed to update team", e),
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
        Ok(false) => return not_found_error("error.team.notFound"),
        Err(e) => {
            return internal_error("Failed to check team existence", e);
        }
    }

    match super::is_team_owner(&state, &team_id.to_string(), &jwt_user.sub.to_string()) {
        Ok(true) => (),
        Ok(false) => {
            if !jwt_user.is_admin() {
                return (StatusCode::FORBIDDEN, Json("error.user.auth.forbidden")).into_response();
            }
        }
        Err(e) => {
            return internal_error("Failed to check team ownership", e);
        }
    }

    match super::delete_team(&state, &team_id.to_string()) {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(e) => internal_error("Failed to delete team", e),
    }
}

#[utoipa::path(
    delete,
    path = "/v1/teams/{team_id}/members/{member_id}",
    tag = "Teams",
    responses(
        (status = 204, description = "Member removed successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Team member not found"),
    ),
)]
pub async fn remove_member(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path((team_id, member_id)): Path<(Ulid, Ulid)>,
) -> impl IntoResponse {
    let team_id = team_id.to_string();
    let member_id = member_id.to_string();
    let requester_id = jwt_user.sub.to_string();

    let can_manage = if requester_id == member_id || jwt_user.is_admin() {
        true
    } else {
        match super::is_team_owner_or_moderator(&state, &team_id, &requester_id) {
            Ok(res) => res,
            Err(e) => {
                return internal_error("Failed to check team permissions", e);
            }
        }
    };

    if !can_manage {
        return (StatusCode::FORBIDDEN, Json("error.user.auth.forbidden")).into_response();
    }

    let member = match super::get_team_member(&state, &team_id, &member_id) {
        Ok(Some(member)) => member,
        Ok(None) => {
            return not_found_error("error.team.member.notFound");
        }
        Err(e) => {
            return internal_error("Failed to fetch team member", e);
        }
    };

    if requester_id != member.user_id
        && let Ok(true) = super::is_team_owner(&state, &team_id, &member.user_id)
    {
        return (
            StatusCode::FORBIDDEN,
            Json("error.team.member.cannotRemoveOwner"),
        )
            .into_response();
    }

    if let Err(e) = super::remove_team_member(&state, &team_id, &member.user_id) {
        return internal_error("Failed to remove team member", e);
    }

    StatusCode::NO_CONTENT.into_response()
}

#[utoipa::path(
    patch,
    path = "/v1/teams/{team_id}/members/{member_id}",
    tag = "Teams",
    request_body = UpdateMemberRolePayload,
    responses(
        (status = 200, description = "Member role updated"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Team member not found"),
    ),
)]
pub async fn update_member_role(
    State(state): State<GlobalState>,
    jwt_user: JwtClaims,
    Path((team_id, member_id)): Path<(Ulid, Ulid)>,
    Json(payload): Json<UpdateMemberRolePayload>,
) -> impl IntoResponse {
    let team_id = team_id.to_string();
    let member_id = member_id.to_string();
    let requester_id = jwt_user.sub.to_string();

    match super::get_team_member(&state, &team_id, &member_id) {
        Ok(Some(_)) => {}
        Ok(None) => {
            return not_found_error("error.team.member.notFound");
        }
        Err(e) => {
            return internal_error("Failed to fetch team member", e);
        }
    }

    if !jwt_user.is_admin() {
        let actor_role = match super::get_member_role(&state, &team_id, &requester_id) {
            Ok(Some(role)) => role,
            Ok(None) => {
                return (StatusCode::FORBIDDEN, Json("error.user.auth.forbidden")).into_response();
            }
            Err(e) => {
                return internal_error("Failed to fetch requester role", e);
            }
        };

        if actor_role != TeamRole::Owner {
            return (StatusCode::FORBIDDEN, Json("error.user.auth.forbidden")).into_response();
        }
    }

    if let Err(e) = super::update_member_role(&state, &team_id, &member_id, payload.role) {
        return internal_error("Failed to update member role", e);
    }

    StatusCode::OK.into_response()
}
