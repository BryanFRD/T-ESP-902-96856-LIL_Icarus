#[cfg(test)]
mod tests {
    use crate::api::events::logging::{
        RoutePattern, infer_delete_event, infer_get_event, infer_patch_event, infer_post_event,
    };
    use crate::api::events::{EventAction, ResourceType};

    // ===== RoutePattern tests =====

    #[test]
    fn test_route_pattern_parses_path() {
        let rp = RoutePattern::new("GET", "/v1/teams/123/company");
        assert_eq!(rp.method, "GET");
        assert_eq!(rp.get(0), Some("teams"));
        assert_eq!(rp.get(1), Some("123"));
        assert_eq!(rp.get(2), Some("company"));
        assert_eq!(rp.get(3), None);
    }

    #[test]
    fn test_route_pattern_empty_path() {
        let rp = RoutePattern::new("GET", "/v1");
        assert_eq!(rp.get(0), None);
    }

    // ===== POST inference =====

    #[test]
    fn test_infer_post_login() {
        let rp = RoutePattern::new("POST", "/v1/auth/login");
        let result = infer_post_event(&rp);
        let (action, rt, _) = result.unwrap();
        assert_eq!(action, EventAction::Login);
        assert_eq!(rt, ResourceType::Auth);
    }

    #[test]
    fn test_infer_post_register() {
        let rp = RoutePattern::new("POST", "/v1/auth/register");
        let (action, rt, _) = infer_post_event(&rp).unwrap();
        assert_eq!(action, EventAction::Register);
        assert_eq!(rt, ResourceType::Auth);
    }

    #[test]
    fn test_infer_post_logout() {
        let rp = RoutePattern::new("POST", "/v1/auth/logout");
        let (action, _, _) = infer_post_event(&rp).unwrap();
        assert_eq!(action, EventAction::Logout);
    }

    #[test]
    fn test_infer_post_password_reset() {
        let rp = RoutePattern::new("POST", "/v1/auth/reset-password");
        let (action, _, _) = infer_post_event(&rp).unwrap();
        assert_eq!(action, EventAction::PasswordReset);
    }

    #[test]
    fn test_infer_post_avatar_upload() {
        let rp = RoutePattern::new("POST", "/v1/users/me/avatar");
        let (action, rt, _) = infer_post_event(&rp).unwrap();
        assert_eq!(action, EventAction::AvatarUpload);
        assert_eq!(rt, ResourceType::Avatar);
    }

    #[test]
    fn test_infer_post_create_team() {
        let rp = RoutePattern::new("POST", "/v1/teams/me");
        let (action, rt, _) = infer_post_event(&rp).unwrap();
        assert_eq!(action, EventAction::Create);
        assert_eq!(rt, ResourceType::Team);
    }

    #[test]
    fn test_infer_post_create_company() {
        let rp = RoutePattern::new("POST", "/v1/teams/t1/company");
        let (action, rt, id) = infer_post_event(&rp).unwrap();
        assert_eq!(action, EventAction::Create);
        assert_eq!(rt, ResourceType::Company);
        assert!(id.unwrap().contains("t1"));
    }

    #[test]
    fn test_infer_post_team_invitation() {
        let rp = RoutePattern::new("POST", "/v1/teams/t1/invitations");
        let (action, rt, _) = infer_post_event(&rp).unwrap();
        assert_eq!(action, EventAction::InvitationCreate);
        assert_eq!(rt, ResourceType::Invitation);
    }

    #[test]
    fn test_infer_post_join_invitation() {
        let rp = RoutePattern::new("POST", "/v1/teams/invitations/join");
        let (action, _, _) = infer_post_event(&rp).unwrap();
        assert_eq!(action, EventAction::InvitationConsume);
    }

    #[test]
    fn test_infer_post_create_audit() {
        let rp = RoutePattern::new("POST", "/v1/audits/me");
        let (action, rt, _) = infer_post_event(&rp).unwrap();
        assert_eq!(action, EventAction::Create);
        assert_eq!(rt, ResourceType::Audit);
    }

    #[test]
    fn test_infer_post_create_ticket() {
        let rp = RoutePattern::new("POST", "/v1/audits/a1/tickets");
        let (action, rt, id) = infer_post_event(&rp).unwrap();
        assert_eq!(action, EventAction::Create);
        assert_eq!(rt, ResourceType::Ticket);
        assert!(id.unwrap().contains("a1"));
    }

    #[test]
    fn test_infer_post_create_column() {
        let rp = RoutePattern::new("POST", "/v1/audits/a1/columns");
        let (action, rt, _) = infer_post_event(&rp).unwrap();
        assert_eq!(action, EventAction::Create);
        assert_eq!(rt, ResourceType::KanbanColumn);
    }

    #[test]
    fn test_infer_post_create_comment() {
        let rp = RoutePattern::new("POST", "/v1/audits/a1/tickets/t1/comments");
        let (action, rt, id) = infer_post_event(&rp).unwrap();
        assert_eq!(action, EventAction::Create);
        assert_eq!(rt, ResourceType::TicketComment);
        let id_str = id.unwrap();
        assert!(id_str.contains("a1"));
        assert!(id_str.contains("t1"));
    }

    #[test]
    fn test_infer_post_unknown_path() {
        let rp = RoutePattern::new("POST", "/v1/unknown/path");
        assert!(infer_post_event(&rp).is_none());
    }

    // ===== GET inference =====

    #[test]
    fn test_infer_get_confirm_email() {
        let rp = RoutePattern::new("GET", "/v1/auth/confirm-email");
        let (action, _, _) = infer_get_event(&rp).unwrap();
        assert_eq!(action, EventAction::EmailConfirm);
    }

    #[test]
    fn test_infer_get_user_me() {
        let rp = RoutePattern::new("GET", "/v1/users/me");
        let (action, rt, _) = infer_get_event(&rp).unwrap();
        assert_eq!(action, EventAction::Read);
        assert_eq!(rt, ResourceType::User);
    }

    #[test]
    fn test_infer_get_specific_user() {
        let rp = RoutePattern::new("GET", "/v1/users/u123");
        let (action, rt, id) = infer_get_event(&rp).unwrap();
        assert_eq!(action, EventAction::Read);
        assert_eq!(rt, ResourceType::User);
        assert!(id.unwrap().contains("u123"));
    }

    #[test]
    fn test_infer_get_teams_me() {
        let rp = RoutePattern::new("GET", "/v1/teams/me");
        let (action, rt, _) = infer_get_event(&rp).unwrap();
        assert_eq!(action, EventAction::Read);
        assert_eq!(rt, ResourceType::Team);
    }

    #[test]
    fn test_infer_get_team_by_id() {
        let rp = RoutePattern::new("GET", "/v1/teams/t1");
        let (action, rt, _) = infer_get_event(&rp).unwrap();
        assert_eq!(action, EventAction::Read);
        assert_eq!(rt, ResourceType::Team);
    }

    #[test]
    fn test_infer_get_board() {
        let rp = RoutePattern::new("GET", "/v1/audits/a1/board");
        let (action, rt, _) = infer_get_event(&rp).unwrap();
        assert_eq!(action, EventAction::Read);
        assert_eq!(rt, ResourceType::Audit);
    }

    #[test]
    fn test_infer_get_unknown() {
        let rp = RoutePattern::new("GET", "/v1/unknown");
        assert!(infer_get_event(&rp).is_none());
    }

    // ===== PATCH inference =====

    #[test]
    fn test_infer_patch_user_me() {
        let rp = RoutePattern::new("PATCH", "/v1/users/me");
        let (action, rt, _) = infer_patch_event(&rp).unwrap();
        assert_eq!(action, EventAction::Update);
        assert_eq!(rt, ResourceType::User);
    }

    #[test]
    fn test_infer_patch_team() {
        let rp = RoutePattern::new("PATCH", "/v1/teams/t1");
        let (action, rt, _) = infer_patch_event(&rp).unwrap();
        assert_eq!(action, EventAction::Update);
        assert_eq!(rt, ResourceType::Team);
    }

    #[test]
    fn test_infer_patch_company() {
        let rp = RoutePattern::new("PATCH", "/v1/teams/t1/company/c1");
        let (action, rt, id) = infer_patch_event(&rp).unwrap();
        assert_eq!(action, EventAction::Update);
        assert_eq!(rt, ResourceType::Company);
        let id_str = id.unwrap();
        assert!(id_str.contains("t1"));
        assert!(id_str.contains("c1"));
    }

    #[test]
    fn test_infer_patch_team_member() {
        let rp = RoutePattern::new("PATCH", "/v1/teams/t1/members/u1");
        let (action, rt, _) = infer_patch_event(&rp).unwrap();
        assert_eq!(action, EventAction::Update);
        assert_eq!(rt, ResourceType::TeamMember);
    }

    #[test]
    fn test_infer_patch_ticket() {
        let rp = RoutePattern::new("PATCH", "/v1/audits/a1/tickets/t1");
        let (action, rt, _) = infer_patch_event(&rp).unwrap();
        assert_eq!(action, EventAction::Update);
        assert_eq!(rt, ResourceType::Ticket);
    }

    #[test]
    fn test_infer_patch_comment() {
        let rp = RoutePattern::new("PATCH", "/v1/audits/a1/tickets/t1/comments/c1");
        let (action, rt, _) = infer_patch_event(&rp).unwrap();
        assert_eq!(action, EventAction::Update);
        assert_eq!(rt, ResourceType::TicketComment);
    }

    #[test]
    fn test_infer_patch_column() {
        let rp = RoutePattern::new("PATCH", "/v1/audits/a1/columns/col1");
        let (action, rt, _) = infer_patch_event(&rp).unwrap();
        assert_eq!(action, EventAction::Update);
        assert_eq!(rt, ResourceType::KanbanColumn);
    }

    #[test]
    fn test_infer_patch_unknown() {
        let rp = RoutePattern::new("PATCH", "/v1/unknown/path");
        assert!(infer_patch_event(&rp).is_none());
    }

    // ===== DELETE inference =====

    #[test]
    fn test_infer_delete_avatar() {
        let rp = RoutePattern::new("DELETE", "/v1/users/me/avatar");
        let (action, rt, _) = infer_delete_event(&rp).unwrap();
        assert_eq!(action, EventAction::AvatarDelete);
        assert_eq!(rt, ResourceType::Avatar);
    }

    #[test]
    fn test_infer_delete_user_me() {
        let rp = RoutePattern::new("DELETE", "/v1/users/me");
        let (action, rt, _) = infer_delete_event(&rp).unwrap();
        assert_eq!(action, EventAction::Delete);
        assert_eq!(rt, ResourceType::User);
    }

    #[test]
    fn test_infer_delete_team() {
        let rp = RoutePattern::new("DELETE", "/v1/teams/t1");
        let (action, rt, _) = infer_delete_event(&rp).unwrap();
        assert_eq!(action, EventAction::Delete);
        assert_eq!(rt, ResourceType::Team);
    }

    #[test]
    fn test_infer_delete_company() {
        let rp = RoutePattern::new("DELETE", "/v1/teams/t1/company/c1");
        let (action, rt, _) = infer_delete_event(&rp).unwrap();
        assert_eq!(action, EventAction::Delete);
        assert_eq!(rt, ResourceType::Company);
    }

    #[test]
    fn test_infer_delete_team_member() {
        let rp = RoutePattern::new("DELETE", "/v1/teams/t1/members/u1");
        let (action, rt, _) = infer_delete_event(&rp).unwrap();
        assert_eq!(action, EventAction::Delete);
        assert_eq!(rt, ResourceType::TeamMember);
    }

    #[test]
    fn test_infer_delete_cancel_team_invitation() {
        let rp = RoutePattern::new("DELETE", "/v1/teams/t1/invitations/tok1");
        let (action, rt, _) = infer_delete_event(&rp).unwrap();
        assert_eq!(action, EventAction::InvitationCancel);
        assert_eq!(rt, ResourceType::Invitation);
    }

    #[test]
    fn test_infer_delete_audit() {
        let rp = RoutePattern::new("DELETE", "/v1/audits/a1");
        let (action, rt, _) = infer_delete_event(&rp).unwrap();
        assert_eq!(action, EventAction::Delete);
        assert_eq!(rt, ResourceType::Audit);
    }

    #[test]
    fn test_infer_delete_audit_client() {
        let rp = RoutePattern::new("DELETE", "/v1/audits/a1/clients/u1");
        let (action, rt, _) = infer_delete_event(&rp).unwrap();
        assert_eq!(action, EventAction::Delete);
        assert_eq!(rt, ResourceType::AuditClient);
    }

    #[test]
    fn test_infer_delete_ticket() {
        let rp = RoutePattern::new("DELETE", "/v1/audits/a1/tickets/t1");
        let (action, rt, _) = infer_delete_event(&rp).unwrap();
        assert_eq!(action, EventAction::Delete);
        assert_eq!(rt, ResourceType::Ticket);
    }

    #[test]
    fn test_infer_delete_comment() {
        let rp = RoutePattern::new("DELETE", "/v1/audits/a1/tickets/t1/comments/c1");
        let (action, rt, _) = infer_delete_event(&rp).unwrap();
        assert_eq!(action, EventAction::Delete);
        assert_eq!(rt, ResourceType::TicketComment);
    }

    #[test]
    fn test_infer_delete_attachment() {
        let rp = RoutePattern::new("DELETE", "/v1/audits/a1/tickets/t1/attachments/att1");
        let (action, rt, _) = infer_delete_event(&rp).unwrap();
        assert_eq!(action, EventAction::Delete);
        assert_eq!(rt, ResourceType::TicketAttachment);
    }

    #[test]
    fn test_infer_delete_column() {
        let rp = RoutePattern::new("DELETE", "/v1/audits/a1/columns/col1");
        let (action, rt, _) = infer_delete_event(&rp).unwrap();
        assert_eq!(action, EventAction::Delete);
        assert_eq!(rt, ResourceType::KanbanColumn);
    }

    #[test]
    fn test_infer_delete_label() {
        let rp = RoutePattern::new("DELETE", "/v1/audits/a1/labels/l1");
        let (action, rt, _) = infer_delete_event(&rp).unwrap();
        assert_eq!(action, EventAction::Delete);
        assert_eq!(rt, ResourceType::TicketLabel);
    }

    #[test]
    fn test_infer_delete_unknown() {
        let rp = RoutePattern::new("DELETE", "/v1/unknown");
        assert!(infer_delete_event(&rp).is_none());
    }

    // ===== POST audit invitation =====

    #[test]
    fn test_infer_post_audit_invitation() {
        let rp = RoutePattern::new("POST", "/v1/audits/a1/invite");
        let (action, rt, _) = infer_post_event(&rp).unwrap();
        assert_eq!(action, EventAction::InvitationCreate);
        assert_eq!(rt, ResourceType::AuditInvitation);
    }

    #[test]
    fn test_infer_post_accept_audit_invitation() {
        let rp = RoutePattern::new("POST", "/v1/audits/invite/tok1/accept");
        let (action, rt, _) = infer_post_event(&rp).unwrap();
        assert_eq!(action, EventAction::InvitationConsume);
        assert_eq!(rt, ResourceType::AuditInvitation);
    }

    #[test]
    fn test_infer_delete_cancel_audit_invitation() {
        let rp = RoutePattern::new("DELETE", "/v1/audits/a1/invite/inv1");
        let (action, rt, _) = infer_delete_event(&rp).unwrap();
        assert_eq!(action, EventAction::InvitationCancel);
        assert_eq!(rt, ResourceType::AuditInvitation);
    }

    #[test]
    fn test_infer_delete_ticket_label() {
        let rp = RoutePattern::new("DELETE", "/v1/audits/a1/tickets/t1/labels/l1");
        let (action, rt, _) = infer_delete_event(&rp).unwrap();
        assert_eq!(action, EventAction::Delete);
        assert_eq!(rt, ResourceType::TicketLabel);
    }
}
