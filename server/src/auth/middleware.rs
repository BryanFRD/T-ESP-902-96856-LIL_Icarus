use axum::{
    Json, RequestPartsExt,
    extract::{FromRequestParts, Request},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};
use tower_cookies::Cookies;
use tracing::error;

use super::models::JwtClaims;

pub async fn middleware(_jwt: JwtClaims, req: Request, next: Next) -> impl IntoResponse {
    next.run(req).await
}

pub async fn admin_middleware(jwt: JwtClaims, req: Request, next: Next) -> impl IntoResponse {
    if !jwt.is_admin() {
        return (StatusCode::FORBIDDEN, Json("error.user.auth.forbidden")).into_response();
    }
    next.run(req).await
}

pub async fn pentester_middleware(jwt: JwtClaims, req: Request, next: Next) -> impl IntoResponse {
    if !jwt.is_pentester() {
        return (StatusCode::FORBIDDEN, Json("error.user.auth.forbidden")).into_response();
    }
    next.run(req).await
}

impl<S: Send + Sync> FromRequestParts<S> for JwtClaims {
    type Rejection = (StatusCode, Json<&'static str>);

    async fn from_request_parts(
        parts: &mut http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let cookies = match parts.extract::<Cookies>().await {
            Ok(cookies) => cookies,
            Err(e) => {
                error!("Failed to extract cookies: {:?}", e);
                return Err((StatusCode::UNAUTHORIZED, Json("error.user.auth.unauthorized")));
            }
        };

        let auth_cookie = cookies
            .get("auth")
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json("error.user.auth.unauthorized")))?;

        let jwt_secret = std::env::var("JWT_SECRET").map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("error.server.internalError"),
            )
        })?;

        let jwt = JwtClaims::from_jwt(auth_cookie.value(), &jwt_secret);

        if !jwt.is_valid() {
            return Err((StatusCode::UNAUTHORIZED, Json("error.user.auth.unauthorized")));
        }

        Ok(jwt)
    }
}
