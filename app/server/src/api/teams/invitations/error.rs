use crate::api::teams::TeamError;
use diesel::{r2d2::PoolError, result::Error as DieselError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InvitationError {
    #[error("Error while getting database connection: {0}")]
    DatabaseConnectionError(#[from] PoolError),
    #[error("Error while querying the database: {0}")]
    DatabaseError(#[from] DieselError),
    #[error("Not authorized")]
    NotAuthorized,
    #[error("Team not found")]
    TeamNotFound,
    #[error("Invitation not found")]
    NotFound,
    #[error("Invitation expired")]
    Expired,
    #[error("Invitation canceled")]
    Canceled,
    #[error("Invitation uses exhausted")]
    Exhausted,
}

impl From<TeamError> for InvitationError {
    fn from(_: TeamError) -> Self {
        InvitationError::NotAuthorized
    }
}
