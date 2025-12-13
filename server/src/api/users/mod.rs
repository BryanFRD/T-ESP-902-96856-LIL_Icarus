pub mod endpoints;
mod error;
mod models;
mod services;

pub use error::UserError;
pub use models::{User, UserRole};

#[cfg(test)]
pub use models::UpdatePayload;
pub use services::*;
