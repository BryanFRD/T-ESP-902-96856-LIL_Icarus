use std::io;

use axum::extract::multipart::MultipartError;
use diesel::{r2d2::PoolError, result::Error as DieselError};
use image::ImageError;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("Error while getting database connection: {0}")]
    DatabaseConnectionError(#[from] PoolError),
    #[error("Error while querying the database: {0}")]
    DatabaseError(#[from] DieselError),
    #[error("User not found")]
    UserNotFound,
    #[error("Payload isn't valid: {0}")]
    PayloadValidationError(#[from] ValidationErrors),
    #[error("{0}")]
    UpdatePayloadError(String),
    #[error("Error with IO: {0}")]
    IoError(#[from] io::Error),
    #[error("Multipart Error")]
    MultipartConvertedError(#[from] MultipartError),
    #[error("{0}")]
    MultipartError(String),
    #[error("Avatar not found")]
    AvatarNotFound,
    #[error("Image handling error: {0}")]
    ImageHandlingError(#[from] ImageError),
}
