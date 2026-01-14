use diesel::{r2d2::PoolError, result::Error as DieselError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EventError {
    #[error("Error while getting database connection: {0}")]
    DatabaseConnection(#[from] PoolError),

    #[error("Error while querying the database: {0}")]
    Database(#[from] DieselError),
}
