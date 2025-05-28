use crate::auth;
use crate::auth::{LoginPayload, RegisterPayload};
use crate::status;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        auth::endpoints::login,
        auth::endpoints::register,
        auth::endpoints::me,
        auth::endpoints::update_me,
        auth::endpoints::logout,
        status::endpoints::get_status,
    ),
    components(
        schemas(LoginPayload, RegisterPayload),
    ),
    tags(
        (name = "Auth", description = "Authentication endpoints"),
        (name = "User", description = "User management endpoints"),
        (name = "Status", description = "Application status endpoints"),
    ),
    security(
        ("cookieAuth" = [])
    ),
)]
pub struct ApiDoc;
