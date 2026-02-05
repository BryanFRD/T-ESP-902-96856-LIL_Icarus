use crate::api::audits::AuditError;
use crate::api::users::UserError;
use diesel::r2d2::PoolError;
use diesel::result::Error as DieselError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuditInvitationError {
    #[error("Error while getting database connection: {0}")]
    DatabaseConnection(#[from] PoolError),

    #[error("Error while querying the database: {0}")]
    Database(#[from] DieselError),

    #[error("Audit error: {0}")]
    AuditError(#[from] AuditError),

    #[error("User error: {0}")]
    UserError(#[from] UserError),

    #[error("Not authorized")]
    NotAuthorized,

    #[error("Invitation not found")]
    NotFound,

    #[error("Invitation expired")]
    Expired,

    #[error("Invitation canceled")]
    Canceled,

    #[error("Invitation already used")]
    AlreadyUsed,

    #[error("Email already in audit")]
    EmailAlreadyInAudit,

    #[error("Email mismatch")]
    EmailMismatch,

    #[error("Token invalid: {0}")]
    TokenInvalid(String),

    #[error("Authentication required")]
    AuthenticationRequired,

    #[error("Missing account details")]
    MissingAccountDetails,

    #[error("Username already taken")]
    UsernameAlreadyTaken,

    #[error("Password too short")]
    PasswordTooShort,

    #[error("Email send error: {0}")]
    EmailSendError(#[from] resend_rs::Error),

    #[error("JWT generation failed")]
    JwtGenerationFailed,

    #[error("Encryption error: {0}")]
    EncryptionError(#[from] shared::encryption::EncryptionError),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Invalid base64 encoding: {0}")]
    Base64Error(#[from] base64::DecodeError),
}
