use diesel::{r2d2::PoolError, result::Error as DieselError};
use thiserror::Error;

use crate::api::audits::AuditError;

#[derive(Debug, Error)]
pub enum AuditMessageError {
    #[error("Error while getting database connection: {0}")]
    DatabaseConnection(#[from] PoolError),

    #[error("Error while querying the database: {0}")]
    Database(#[from] DieselError),

    #[error("Audit error: {0}")]
    Audit(#[from] AuditError),

    #[error("Not authorized")]
    NotAuthorized,

    #[error("Message not found")]
    MessageNotFound,

    #[error("Attachment not found")]
    AttachmentNotFound,

    #[error("Audit is archived and cannot be modified")]
    AuditArchived,

    #[error("File is too large")]
    FileTooLarge,

    #[error("File storage error: {0}")]
    FileStorage(String),

    #[error("Encryption error: {0}")]
    EncryptionError(#[from] shared::encryption::EncryptionError),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Invalid base64 encoding: {0}")]
    Base64Error(#[from] base64::DecodeError),
}
