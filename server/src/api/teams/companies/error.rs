use crate::api::teams::TeamError;
use diesel::{r2d2::PoolError, result::Error as DieselError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompanyError {
    #[error("Error while getting database connection: {0}")]
    DatabaseConnection(#[from] PoolError),
    #[error("Error while querying the database: {0}")]
    Database(#[from] DieselError),
    #[error("Not authorized")]
    NotAuthorized,
    #[error("Team not found")]
    TeamNotFound,
    #[error("Company not found")]
    NotFound,
    #[error("Company with this SIRET already exists in the team")]
    DuplicateSiret,
    #[error("Unauthorized - team membership required")]
    NotTeamMember,
}

impl From<TeamError> for CompanyError {
    fn from(_: TeamError) -> Self {
        Self::NotAuthorized
    }
}
