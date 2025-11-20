use diesel::{r2d2::PoolError, result::Error as DieselError};
use thiserror::Error;

use crate::api::users::UserError;

#[derive(Debug, Error)]
pub enum TeamError {
    #[error("Error while getting database connection: {0}")]
    DatabaseConnection(#[from] PoolError),
    #[error("Error while querying the database: {0}")]
    Database(#[from] DieselError),
    #[error("Error while handling user: {0}")]
    User(#[from] UserError),
}
