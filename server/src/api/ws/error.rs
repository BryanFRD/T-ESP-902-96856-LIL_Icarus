use serde::Serialize;

#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum WsError {
    #[error("Authentication failed: {0}")]
    AuthFailed(String),

    #[error("Room access denied: {0}")]
    RoomAccessDenied(String),

    #[error("Step-up authentication required")]
    StepUpRequired,

    #[error("Step-up authentication invalid")]
    StepUpInvalid,

    #[error("Invalid payload: {0}")]
    InvalidPayload(String),

    #[error("Database error")]
    DatabaseError,

    #[error("Audit not found")]
    AuditNotFound,

    #[error("Audit is archived")]
    AuditArchived,
}

impl WsError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::AuthFailed(_) => "AUTH_FAILED",
            Self::RoomAccessDenied(_) => "ROOM_ACCESS_DENIED",
            Self::StepUpRequired => "AUDIT_STEP_UP_REQUIRED",
            Self::StepUpInvalid => "AUDIT_STEP_UP_INVALID",
            Self::InvalidPayload(_) => "INVALID_PAYLOAD",
            Self::DatabaseError => "DATABASE_ERROR",
            Self::AuditNotFound => "AUDIT_NOT_FOUND",
            Self::AuditArchived => "AUDIT_ARCHIVED",
        }
    }

    pub fn to_response(&self) -> WsErrorResponse {
        WsErrorResponse {
            code: self.code().to_string(),
            message: self.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct WsErrorResponse {
    pub code: String,
    pub message: String,
}
