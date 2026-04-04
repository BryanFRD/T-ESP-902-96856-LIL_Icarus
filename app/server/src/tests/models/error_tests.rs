#[allow(clippy::too_many_lines)]
#[cfg(test)]
mod tests {
    use crate::api::audits::AuditError;
    use crate::api::audits::invitations::error::AuditInvitationError;
    use crate::api::audits::messages::error::AuditMessageError;
    use crate::api::audits::tickets::error::TicketError;
    use crate::api::agents::error::AgentError;
    use crate::api::events::error::EventError;
    use crate::api::teams::companies::error::CompanyError;
    use crate::api::teams::invitations::error::InvitationError;
    use crate::api::teams::TeamError;

    // TicketError Display
    #[test]
    fn test_ticket_error_display_not_found() {
        assert_eq!(TicketError::NotFound.to_string(), "Ticket not found");
    }

    #[test]
    fn test_ticket_error_display_column_not_found() {
        assert_eq!(TicketError::ColumnNotFound.to_string(), "Column not found");
    }

    #[test]
    fn test_ticket_error_display_label_not_found() {
        assert_eq!(TicketError::LabelNotFound.to_string(), "Label not found");
    }

    #[test]
    fn test_ticket_error_display_comment_not_found() {
        assert_eq!(TicketError::CommentNotFound.to_string(), "Comment not found");
    }

    #[test]
    fn test_ticket_error_display_attachment_not_found() {
        assert_eq!(TicketError::AttachmentNotFound.to_string(), "Attachment not found");
    }

    #[test]
    fn test_ticket_error_display_parent_not_found() {
        assert_eq!(TicketError::ParentNotFound.to_string(), "Parent ticket not found");
    }

    #[test]
    fn test_ticket_error_display_self_parent() {
        assert_eq!(TicketError::SelfParent.to_string(), "Ticket cannot be its own parent");
    }

    #[test]
    fn test_ticket_error_display_circular_parent() {
        assert_eq!(TicketError::CircularParent.to_string(), "Circular parent reference detected");
    }

    #[test]
    fn test_ticket_error_display_archived() {
        assert_eq!(TicketError::AuditArchived.to_string(), "Audit is archived and cannot be modified");
    }

    #[test]
    fn test_ticket_error_display_must_be_team_member() {
        assert_eq!(TicketError::MustBeTeamMember.to_string(), "Must be a member of the audit team");
    }

    #[test]
    fn test_ticket_error_display_must_be_owner() {
        assert_eq!(TicketError::MustBeOwnerOrAdmin.to_string(), "Must be team owner, moderator, or admin");
    }

    #[test]
    fn test_ticket_error_display_not_comment_author() {
        assert_eq!(TicketError::NotCommentAuthor.to_string(), "Only the comment author can edit this comment");
    }

    #[test]
    fn test_ticket_error_display_column_mismatch() {
        assert_eq!(TicketError::ColumnAuditMismatch.to_string(), "Column does not belong to this audit");
    }

    #[test]
    fn test_ticket_error_display_label_mismatch() {
        assert_eq!(TicketError::LabelAuditMismatch.to_string(), "Label does not belong to this audit");
    }

    #[test]
    fn test_ticket_error_display_label_already_attached() {
        assert_eq!(TicketError::LabelAlreadyAttached.to_string(), "Label is already attached to this ticket");
    }

    #[test]
    fn test_ticket_error_display_label_not_attached() {
        assert_eq!(TicketError::LabelNotAttached.to_string(), "Label is not attached to this ticket");
    }

    #[test]
    fn test_ticket_error_display_column_has_tickets() {
        assert_eq!(TicketError::ColumnHasTickets.to_string(), "Column still has tickets and cannot be deleted");
    }

    #[test]
    fn test_ticket_error_display_duplicate_column() {
        assert_eq!(TicketError::DuplicateColumnName.to_string(), "A column with this name already exists in this audit");
    }

    #[test]
    fn test_ticket_error_display_duplicate_label() {
        assert_eq!(TicketError::DuplicateLabelName.to_string(), "A label with this name already exists in this audit");
    }

    #[test]
    fn test_ticket_error_display_file_too_large() {
        assert_eq!(TicketError::FileTooLarge.to_string(), "File is too large");
    }

    #[test]
    fn test_ticket_error_display_invalid_file_type() {
        assert_eq!(TicketError::InvalidFileType.to_string(), "Invalid file type");
    }

    #[test]
    fn test_ticket_error_display_file_storage() {
        let err = TicketError::FileStorage("disk full".into());
        assert_eq!(err.to_string(), "File storage error: disk full");
    }

    #[test]
    fn test_ticket_error_from_team_error_database() {
        let team_err = TeamError::Database(diesel::result::Error::NotFound);
        let ticket_err = TicketError::from(team_err);
        assert!(matches!(ticket_err, TicketError::Database(_)));
    }

    // AuditError Display
    #[test]
    fn test_audit_error_display_variants() {
        assert_eq!(AuditError::NotFound.to_string(), "Audit not found");
        assert_eq!(AuditError::TeamNotFound.to_string(), "Team not found");
        assert_eq!(AuditError::CompanyNotFound.to_string(), "Company not found");
        assert_eq!(AuditError::CompanyTeamMismatch.to_string(), "Company does not belong to the specified team");
        assert_eq!(AuditError::MustBeOwnerOrAdmin.to_string(), "Must be team owner or admin");
        assert_eq!(AuditError::AuditArchived.to_string(), "Audit is archived and cannot be modified");
        assert_eq!(AuditError::EndDateInPast.to_string(), "End date cannot be in the past");
        assert_eq!(AuditError::ClientNotInAudit.to_string(), "Client not in audit");
        assert_eq!(AuditError::StepUpRequired.to_string(), "Step-up authentication required");
        assert_eq!(AuditError::StepUpInvalidToken.to_string(), "Invalid audit access token");
        assert_eq!(AuditError::StepUpExpired.to_string(), "Audit access token expired");
        assert_eq!(AuditError::StepUpInvalidPassword.to_string(), "Invalid password for step-up authentication");
    }

    // AuditMessageError Display
    #[test]
    fn test_audit_message_error_display() {
        assert_eq!(AuditMessageError::NotAuthorized.to_string(), "Not authorized");
        assert_eq!(AuditMessageError::MessageNotFound.to_string(), "Message not found");
        assert_eq!(AuditMessageError::AttachmentNotFound.to_string(), "Attachment not found");
        assert_eq!(AuditMessageError::AuditArchived.to_string(), "Audit is archived and cannot be modified");
        assert_eq!(AuditMessageError::FileTooLarge.to_string(), "File is too large");
        let err = AuditMessageError::FileStorage("error".into());
        assert_eq!(err.to_string(), "File storage error: error");
    }

    // AgentError Display
    #[test]
    fn test_agent_error_display() {
        assert_eq!(AgentError::NotFound.to_string(), "Agent not found");
        assert_eq!(AgentError::NameAlreadyExists.to_string(), "An agent with this name already exists for this audit");
        assert_eq!(AgentError::MustBeOwnerOrAdmin.to_string(), "Must be team owner or admin");
        assert_eq!(AgentError::Revoked.to_string(), "Agent has been revoked");
        assert_eq!(AgentError::InvalidToken.to_string(), "Invalid agent token");
        assert_eq!(AgentError::AgentNotOnline.to_string(), "Agent is not online");
        let err = AgentError::ShellRequestFailed("timeout".into());
        assert_eq!(err.to_string(), "Failed to send shell request: timeout");
        let err = AgentError::KeyloggerRequestFailed("denied".into());
        assert_eq!(err.to_string(), "Failed to send keylogger request: denied");
    }

    // InvitationError Display
    #[test]
    fn test_invitation_error_display() {
        assert_eq!(InvitationError::NotAuthorized.to_string(), "Not authorized");
        assert_eq!(InvitationError::TeamNotFound.to_string(), "Team not found");
        assert_eq!(InvitationError::NotFound.to_string(), "Invitation not found");
        assert_eq!(InvitationError::Expired.to_string(), "Invitation expired");
        assert_eq!(InvitationError::Canceled.to_string(), "Invitation canceled");
        assert_eq!(InvitationError::Exhausted.to_string(), "Invitation uses exhausted");
    }

    #[test]
    fn test_invitation_error_from_team_error() {
        let team_err = TeamError::Database(diesel::result::Error::NotFound);
        let inv_err = InvitationError::from(team_err);
        assert!(matches!(inv_err, InvitationError::NotAuthorized));
    }

    // CompanyError Display
    #[test]
    fn test_company_error_display() {
        assert_eq!(CompanyError::NotAuthorized.to_string(), "Not authorized");
        assert_eq!(CompanyError::TeamNotFound.to_string(), "Team not found");
        assert_eq!(CompanyError::NotFound.to_string(), "Company not found");
        assert_eq!(CompanyError::DuplicateSiret.to_string(), "Company with this SIRET already exists in the team");
        assert_eq!(CompanyError::NotTeamMember.to_string(), "Unauthorized - team membership required");
    }

    #[test]
    fn test_company_error_from_team_error() {
        let team_err = TeamError::Database(diesel::result::Error::NotFound);
        let comp_err = CompanyError::from(team_err);
        assert!(matches!(comp_err, CompanyError::NotAuthorized));
    }

    // EventError Display
    #[test]
    fn test_event_error_display() {
        let err = EventError::Database(diesel::result::Error::NotFound);
        assert!(err.to_string().contains("querying the database"));
    }

    // AuditInvitationError Display
    #[test]
    fn test_audit_invitation_error_display() {
        assert_eq!(AuditInvitationError::NotAuthorized.to_string(), "Not authorized");
        assert_eq!(AuditInvitationError::NotFound.to_string(), "Invitation not found");
        assert_eq!(AuditInvitationError::Expired.to_string(), "Invitation expired");
        assert_eq!(AuditInvitationError::Canceled.to_string(), "Invitation canceled");
        assert_eq!(AuditInvitationError::AlreadyUsed.to_string(), "Invitation already used");
        assert_eq!(AuditInvitationError::EmailAlreadyInAudit.to_string(), "Email already in audit");
        assert_eq!(AuditInvitationError::EmailMismatch.to_string(), "Email mismatch");
        assert_eq!(AuditInvitationError::AuthenticationRequired.to_string(), "Authentication required");
        assert_eq!(AuditInvitationError::MissingAccountDetails.to_string(), "Missing account details");
        assert_eq!(AuditInvitationError::UsernameAlreadyTaken.to_string(), "Username already taken");
        assert_eq!(AuditInvitationError::PasswordTooShort.to_string(), "Password too short");
        assert_eq!(AuditInvitationError::JwtGenerationFailed.to_string(), "JWT generation failed");
        let err = AuditInvitationError::TokenInvalid("bad".into());
        assert_eq!(err.to_string(), "Token invalid: bad");
    }
}
