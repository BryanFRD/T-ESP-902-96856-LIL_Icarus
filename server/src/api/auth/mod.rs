pub mod endpoints;
pub mod models;
mod services;

pub use crate::middlewares::auth::{admin_middleware, middleware, pentester_middleware};
pub use services::get_routes;
pub use services::hash_password;

pub use models::{ConfirmEmailPayload, JwtClaims, LoginPayload, RegisterPayload};

#[cfg(test)]
pub use models::{EmailConfirmationToken, ResetPasswordPayload, ResetPasswordRequestPayload};
