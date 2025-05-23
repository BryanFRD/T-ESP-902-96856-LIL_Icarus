use axum::{
    Json, RequestPartsExt,
    extract::{FromRequestParts, Request},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};
use tower_cookies::Cookies;

use super::models::JwtClaims;

pub async fn middleware(_jwt: JwtClaims, req: Request, next: Next) -> impl IntoResponse {
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
            Err(_) => return Err((StatusCode::UNAUTHORIZED, Json("Unauthorized"))),
        };

        let auth_cookie = cookies
            .get("auth")
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json("Unauthorized")))?;

        let jwt_secret = std::env::var("JWT_SECRET").map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Internal Server Error"),
            )
        })?;

        let jwt = JwtClaims::from_jwt(auth_cookie.value(), &jwt_secret);

        if !jwt.is_valid() {
            return Err((StatusCode::UNAUTHORIZED, Json("Unauthorized")));
        }

        Ok(jwt)
    }
}
