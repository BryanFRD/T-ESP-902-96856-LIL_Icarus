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
        let cookies = parts.extract::<Cookies>().await.unwrap();

        let auth_cookie = cookies
            .get("auth")
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, Json("Unauthorized")))?;

        let jwt = JwtClaims::from(auth_cookie.value().to_string());

        if !jwt.is_valid() {
            return Err((StatusCode::UNAUTHORIZED, Json("Unauthorized")));
        }

        Ok(jwt)
    }
}
