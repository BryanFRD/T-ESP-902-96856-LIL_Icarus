use thiserror::Error;
use diesel::{r2d2::PoolError, result::Error as DieselError};

use crate::users::UserError;

#[derive(Debug, Error)]
pub enum TeamError {
    #[error("Error while getting database connection: {0}")]
    DatabaseConnectionError(#[from] PoolError),
    #[error("Error while querying the database: {0}")]
    DatabaseError(#[from] DieselError),
    #[error("Error while handling user: {0}")]
    UserError(#[from] UserError),
}
