use diesel::{r2d2::PoolError, result::Error as DieselError};
use thiserror::Error;

use crate::api::audits::AuditError;

#[derive(Debug, Error)]
pub enum AgentError {
    #[error("Error while getting database connection: {0}")]
    DatabaseConnection(#[from] PoolError),

    #[error("Error while querying the database: {0}")]
    Database(#[from] DieselError),

    #[error("Audit error: {0}")]
    Audit(#[from] AuditError),

    #[error("Agent not found")]
    NotFound,

    #[error("An agent with this name already exists for this audit")]
    NameAlreadyExists,

    #[error("Must be team owner or admin")]
    MustBeOwnerOrAdmin,

    #[error("Agent has been revoked")]
    Revoked,

    #[error("Invalid agent token")]
    InvalidToken,

    #[error("Agent is not online")]
    AgentNotOnline,

    #[error("Failed to send shell request: {0}")]
    ShellRequestFailed(String),
}
