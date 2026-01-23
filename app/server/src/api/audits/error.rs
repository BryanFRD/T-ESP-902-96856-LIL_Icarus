use diesel::{r2d2::PoolError, result::Error as DieselError};
use thiserror::Error;

use crate::api::teams::TeamError;

#[derive(Debug, Error)]
pub enum AuditError {
    #[error("Error while getting database connection: {0}")]
    DatabaseConnection(#[from] PoolError),

    #[error("Error while querying the database: {0}")]
    Database(#[from] DieselError),

    #[error("Team error: {0}")]
    Team(#[from] TeamError),

    #[error("Audit not found")]
    NotFound,

    #[error("Team not found")]
    TeamNotFound,

    #[error("Company not found")]
    CompanyNotFound,

    #[error("Company does not belong to the specified team")]
    CompanyTeamMismatch,

    #[error("Must be team owner or admin")]
    MustBeOwnerOrAdmin,

    #[error("Audit is archived and cannot be modified")]
    AuditArchived,

    #[error("End date cannot be in the past")]
    EndDateInPast,

    #[error("Client not in audit")]
    ClientNotInAudit,

    #[allow(dead_code)]
    #[error("Step-up authentication required")]
    StepUpRequired,

    #[allow(dead_code)]
    #[error("Invalid audit access token")]
    StepUpInvalidToken,

    #[allow(dead_code)]
    #[error("Audit access token expired")]
    StepUpExpired,

    #[allow(dead_code)]
    #[error("Invalid password for step-up authentication")]
    StepUpInvalidPassword,
}
