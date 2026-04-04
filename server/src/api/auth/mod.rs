pub mod endpoints;
pub mod models;
mod services;

pub use crate::middlewares::auth::{admin_middleware, middleware, pentester_middleware};
pub use services::{get_routes, hash_password_argon2id, verify_password_with_migration};

pub use models::{
    ConfirmEmailPayload, JwtClaims, LoginPayload, RegisterPayload, ResetPasswordPayload,
    ResetPasswordRequestPayload,
};

#[cfg(test)]
pub use models::EmailConfirmationToken;

#[cfg(test)]
pub use models::{ConfirmationTokenError, PasswordResetToken};

#[cfg(test)]
pub use services::hash_password;
