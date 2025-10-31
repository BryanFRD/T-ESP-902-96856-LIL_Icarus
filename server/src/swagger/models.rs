use crate::auth;
use crate::auth::{ConfirmEmailPayload, RegisterPayload, LoginPayload};
use crate::status;
use crate::teams;
use crate::teams::{CreateInvitationPayload, CreateTeam, InvitationReturn};
use crate::users;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        auth::endpoints::login,
        auth::endpoints::register,
        auth::endpoints::confirm_email,
        auth::endpoints::logout,
        auth::endpoints::request_password_reset,
        auth::endpoints::reset_password,
        users::endpoints::me,
        users::endpoints::update_me,
        users::endpoints::delete_me,
        users::endpoints::list_users_admin,
        users::endpoints::get_user_admin,
        users::endpoints::update_user_admin,
        users::endpoints::delete_user_admin,
        users::endpoints::upload_avatar,
        users::endpoints::delete_avatar,
        users::endpoints::serve_avatar,
        teams::endpoints::create_team,
        teams::endpoints::get_my_teams,
        teams::endpoints::update_team,
        teams::endpoints::delete_team,
        teams::invitations::endpoints::create_invitation,
        teams::invitations::endpoints::list_invitations,
        teams::invitations::endpoints::delete_invitation,
        teams::invitations::endpoints::consume_invitation,
        status::endpoints::get_status,
    ),
    components(
        schemas(ConfirmEmailPayload, LoginPayload, RegisterPayload, CreateTeam, CreateInvitationPayload, InvitationReturn),
    ),
    tags(
        (name = "Auth", description = "Authentication endpoints"),
        (name = "User", description = "User management endpoints"),
        (name = "Admin", description = "Admin management endpoints"),
        (name = "Status", description = "Application status endpoints"),
        (name = "Teams", description = "Team management endpoints"),
    ),
    security(
        ("cookieAuth" = [])
    ),
)]
pub struct ApiDoc;
