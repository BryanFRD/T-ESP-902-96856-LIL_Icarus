use diesel::{r2d2::PoolError, result::Error as DieselError};
use thiserror::Error;

use crate::api::audits::AuditError;
use crate::api::teams::TeamError;

#[derive(Debug, Error)]
pub enum TicketError {
    #[error("Error while getting database connection: {0}")]
    DatabaseConnection(#[from] PoolError),

    #[error("Error while querying the database: {0}")]
    Database(#[from] DieselError),

    #[error("Audit error: {0}")]
    Audit(#[from] AuditError),

    #[error("Ticket not found")]
    NotFound,

    #[error("Column not found")]
    ColumnNotFound,

    #[error("Label not found")]
    LabelNotFound,

    #[error("Comment not found")]
    CommentNotFound,

    #[error("Attachment not found")]
    AttachmentNotFound,

    #[error("Parent ticket not found")]
    ParentNotFound,

    #[error("Ticket cannot be its own parent")]
    SelfParent,

    #[error("Circular parent reference detected")]
    CircularParent,

    #[error("Audit is archived and cannot be modified")]
    AuditArchived,

    #[error("Must be a member of the audit team")]
    MustBeTeamMember,

    #[error("Must be team owner, moderator, or admin")]
    MustBeOwnerOrAdmin,

    #[error("Only the comment author can edit this comment")]
    NotCommentAuthor,

    #[error("Column does not belong to this audit")]
    ColumnAuditMismatch,

    #[error("Label does not belong to this audit")]
    LabelAuditMismatch,

    #[error("Label is already attached to this ticket")]
    LabelAlreadyAttached,

    #[error("Label is not attached to this ticket")]
    LabelNotAttached,

    #[error("Column still has tickets and cannot be deleted")]
    ColumnHasTickets,

    #[error("A column with this name already exists in this audit")]
    DuplicateColumnName,

    #[error("A label with this name already exists in this audit")]
    DuplicateLabelName,

    #[error("File is too large")]
    FileTooLarge,

    #[error("Invalid file type")]
    InvalidFileType,

    #[error("File storage error: {0}")]
    FileStorage(String),
}

impl From<TeamError> for TicketError {
    fn from(err: TeamError) -> Self {
        match err {
            TeamError::DatabaseConnection(e) => Self::DatabaseConnection(e),
            TeamError::Database(e) => Self::Database(e),
            TeamError::User(_) => Self::Database(DieselError::NotFound),
        }
    }
}
