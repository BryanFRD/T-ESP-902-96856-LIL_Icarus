#[cfg(test)]
mod tests {
    use crate::api::agents::models::{
        AuditAgent, CreateAgentPayload, KeyloggerAction, ShellType, SystemInfoResponse,
    };
    use crate::api::audits::invitations::models::{
        AuditInvitation, AuditInvitationReturn, AuditInvitationToken,
        CreateAuditInvitationPayload,
    };
    use crate::api::status::{Status, StatusEnum};
    use crate::api::teams::companies::models::{
        Company, CompanyReturn, CreateCompanyPayload, UpdateCompanyPayload,
    };
    use crate::api::teams::invitations::models::{
        CreateInvitationPayload, InvitationReturn, TeamInvitation,
    };
    use crate::api::teams::TeamRole;
    use crate::helpers;

    // Status tests
    #[test]
    fn test_status_new_up() {
        let s = Status::new(StatusEnum::Up, "Running".into());
        assert!(matches!(s.status, StatusEnum::Up));
        assert_eq!(s.message, "Running");
    }

    #[test]
    fn test_status_new_maintenance() {
        let s = Status::new(StatusEnum::Maintenance, "Down for maintenance".into());
        assert!(matches!(s.status, StatusEnum::Maintenance));
    }

    #[test]
    fn test_status_enum_serde() {
        let json = serde_json::to_string(&StatusEnum::Up).unwrap();
        let deserialized: StatusEnum = serde_json::from_str(&json).unwrap();
        assert!(matches!(deserialized, StatusEnum::Up));
    }

    // Company tests
    #[test]
    fn test_company_new() {
        let c = Company::new("t1".into(), "ACME".into(), Some("desc".into()), "12345".into());
        assert_eq!(c.team_id, "t1");
        assert_eq!(c.name, "ACME");
        assert_eq!(c.description.as_deref(), Some("desc"));
        assert_eq!(c.siret, "12345");
        assert!(!c.id.is_empty());
    }

    #[test]
    fn test_company_unique_ids() {
        let c1 = Company::new("t".into(), "n".into(), None, "s".into());
        let c2 = Company::new("t".into(), "n".into(), None, "s".into());
        assert_ne!(c1.id, c2.id);
    }

    #[test]
    fn test_company_return_from() {
        let c = Company::new("t1".into(), "ACME".into(), None, "12345".into());
        let ret = CompanyReturn::from(c.clone());
        assert_eq!(ret.id, c.id);
        assert_eq!(ret.name, "ACME");
        assert_eq!(ret.siret, "12345");
    }

    #[test]
    fn test_create_company_payload_serde() {
        let json = r#"{"name":"Corp","siret":"123"}"#;
        let p: CreateCompanyPayload = serde_json::from_str(json).unwrap();
        assert_eq!(p.name, "Corp");
        assert!(p.description.is_none());
    }

    #[test]
    fn test_update_company_payload_serde() {
        let json = r#"{"name":"Updated"}"#;
        let p: UpdateCompanyPayload = serde_json::from_str(json).unwrap();
        assert_eq!(p.name.as_deref(), Some("Updated"));
        assert!(p.siret.is_none());
    }

    // TeamInvitation tests
    #[test]
    fn test_team_invitation_new() {
        let inv = TeamInvitation::new("t1".into(), "u1".into(), 5, None);
        assert_eq!(inv.team_id, "t1");
        assert_eq!(inv.created_by, "u1");
        assert_eq!(inv.max_uses, 5);
        assert_eq!(inv.used_count, 0);
        assert!(!inv.is_canceled);
        assert!(inv.expires_at.is_none());
        assert!(!inv.id.is_empty());
        assert!(!inv.token.is_empty());
        assert_ne!(inv.id, inv.token);
    }

    #[test]
    fn test_team_invitation_with_expiry() {
        let expiry = chrono::Utc::now().naive_utc() + chrono::Duration::hours(24);
        let inv = TeamInvitation::new("t1".into(), "u1".into(), 10, Some(expiry));
        assert!(inv.expires_at.is_some());
    }

    #[test]
    fn test_invitation_return_from() {
        let inv = TeamInvitation::new("t1".into(), "u1".into(), 5, None);
        let ret = InvitationReturn::from(inv.clone());
        assert_eq!(ret.id, inv.id);
        assert_eq!(ret.token, inv.token);
        assert_eq!(ret.max_uses, 5);
        assert!(!ret.is_canceled);
    }

    #[test]
    fn test_create_invitation_payload_serde() {
        let json = r#"{"maxUses":10,"expiresAt":null}"#;
        let p: CreateInvitationPayload = serde_json::from_str(json).unwrap();
        assert_eq!(p.max_uses, 10);
        assert!(p.expires_at.is_none());
    }

    // TeamRole serde
    #[test]
    fn test_team_role_serde() {
        for (role, expected) in [
            (TeamRole::Owner, "\"OWNER\""),
            (TeamRole::Moderator, "\"MODERATOR\""),
            (TeamRole::Member, "\"MEMBER\""),
            (TeamRole::Client, "\"CLIENT\""),
        ] {
            let json = serde_json::to_string(&role).unwrap();
            assert_eq!(json, expected);
            let deserialized: TeamRole = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized, role);
        }
    }

    // AuditAgent tests
    #[test]
    fn test_audit_agent_new() {
        let agent = AuditAgent::new("a1".into(), "agent-1".into(), "hash123".into());
        assert_eq!(agent.audit_id, "a1");
        assert_eq!(agent.name, "agent-1");
        assert_eq!(agent.token_hash, "hash123");
        assert!(!agent.is_revoked);
        assert!(agent.last_connected_at.is_none());
        assert!(!agent.id.is_empty());
    }

    #[test]
    fn test_create_agent_payload_serde() {
        let json = r#"{"name":"my-agent"}"#;
        let p: CreateAgentPayload = serde_json::from_str(json).unwrap();
        assert_eq!(p.name, "my-agent");
    }

    // ShellType tests
    #[test]
    fn test_shell_type_serde() {
        for (st, expected) in [
            (ShellType::Auto, "\"auto\""),
            (ShellType::Bash, "\"bash\""),
            (ShellType::Sh, "\"sh\""),
            (ShellType::Cmd, "\"cmd\""),
            (ShellType::PowerShell, "\"powershell\""),
        ] {
            let json = serde_json::to_string(&st).unwrap();
            assert_eq!(json, expected);
        }
    }

    #[test]
    fn test_shell_type_into_shared() {
        let shared: shared::packets::ShellType = ShellType::Bash.into();
        assert!(matches!(shared, shared::packets::ShellType::Bash));
        let shared: shared::packets::ShellType = ShellType::Cmd.into();
        assert!(matches!(shared, shared::packets::ShellType::Cmd));
        let shared: shared::packets::ShellType = ShellType::PowerShell.into();
        assert!(matches!(shared, shared::packets::ShellType::PowerShell));
        let shared: shared::packets::ShellType = ShellType::Auto.into();
        assert!(matches!(shared, shared::packets::ShellType::Auto));
        let shared: shared::packets::ShellType = ShellType::Sh.into();
        assert!(matches!(shared, shared::packets::ShellType::Sh));
    }

    // KeyloggerAction tests
    #[test]
    fn test_keylogger_action_serde() {
        let json = serde_json::to_string(&KeyloggerAction::Start).unwrap();
        assert_eq!(json, "\"start\"");
        let json = serde_json::to_string(&KeyloggerAction::Stop).unwrap();
        assert_eq!(json, "\"stop\"");
    }

    // SystemInfoResponse tests
    #[test]
    fn test_system_info_response_from() {
        let info = shared::packets::SystemInfo {
            hostname: "host".into(),
            os_name: "Linux".into(),
            os_version: "5.15".into(),
            cpu_name: "AMD".into(),
            cpu_count: 4,
            total_memory_bytes: 8_000_000_000,
            used_memory_bytes: 4_000_000_000,
            uptime_secs: 7200,
        };
        let resp = SystemInfoResponse::from(info);
        assert_eq!(resp.hostname, "host");
        assert_eq!(resp.os_name, "Linux");
        assert_eq!(resp.cpu_count, 4);
        assert_eq!(resp.total_memory_bytes, 8_000_000_000);
        assert_eq!(resp.uptime_secs, 7200);
    }

    // AuditInvitation tests
    #[test]
    fn test_audit_invitation_new() {
        let inv = AuditInvitation::new(
            "a1".into(), "client@example.com".into(), "token123".into(),
            chrono::Utc::now().naive_utc() + chrono::Duration::hours(24), "creator".into(),
        );
        assert_eq!(inv.audit_id, "a1");
        assert_eq!(inv.client_email, "client@example.com");
        assert!(!inv.is_canceled);
        assert!(!inv.is_used);
        assert!(inv.used_at.is_none());
    }

    #[test]
    fn test_audit_invitation_is_expired_future() {
        let future = chrono::Utc::now().naive_utc() + chrono::Duration::hours(24);
        let inv = AuditInvitation::new("a".into(), "e".into(), "t".into(), future, "c".into());
        assert!(!inv.is_expired());
    }

    #[test]
    fn test_audit_invitation_is_expired_past() {
        let past = chrono::Utc::now().naive_utc() - chrono::Duration::hours(1);
        let inv = AuditInvitation::new("a".into(), "e".into(), "t".into(), past, "c".into());
        assert!(inv.is_expired());
    }

    #[test]
    fn test_audit_invitation_return_from() {
        let future = chrono::Utc::now().naive_utc() + chrono::Duration::hours(24);
        let inv = AuditInvitation::new("a1".into(), "e@e.com".into(), "t".into(), future, "c".into());
        let ret = AuditInvitationReturn::from(inv.clone());
        assert_eq!(ret.id, inv.id);
        assert_eq!(ret.audit_id, "a1");
        assert!(!ret.is_canceled);
    }

    // AuditInvitationToken tests
    #[test]
    fn test_audit_invitation_token_new() {
        let token = AuditInvitationToken::new("a1".into(), "client@e.com".into());
        assert_eq!(token.audit_id, "a1");
        assert_eq!(token.client_email, "client@e.com");
        assert!(!token.is_expired());
    }

    #[test]
    fn test_audit_invitation_token_encrypt_decrypt() {
        let key = [0u8; 32];
        let token = AuditInvitationToken::new("audit1".into(), "test@e.com".into());
        let encrypted = token.encrypt(&key).unwrap();
        let decrypted = AuditInvitationToken::decrypt(&encrypted, &key).unwrap();
        assert_eq!(decrypted.audit_id, "audit1");
        assert_eq!(decrypted.client_email, "test@e.com");
    }

    #[test]
    fn test_audit_invitation_token_decrypt_too_short() {
        let key = [0u8; 32];
        let short = base64::Engine::encode(
            &base64::engine::general_purpose::URL_SAFE_NO_PAD, b"short",
        );
        let result = AuditInvitationToken::decrypt(&short, &key);
        assert!(result.is_err());
    }

    // CreateAuditInvitationPayload serde
    #[test]
    fn test_create_audit_invitation_payload_serde() {
        let json = r#"{"clientEmail":"test@e.com","expiresAt":null}"#;
        let p: CreateAuditInvitationPayload = serde_json::from_str(json).unwrap();
        assert_eq!(p.client_email, "test@e.com");
        assert!(p.expires_at.is_none());
    }

    // Helpers tests
    #[test]
    fn test_unauthorized_error_status() {
        let resp = helpers::unauthorized_error();
        assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_not_found_error_status() {
        let resp = helpers::not_found_error("not found");
        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_conflict_error_status() {
        let resp = helpers::conflict_error("conflict");
        assert_eq!(resp.status(), http::StatusCode::CONFLICT);
    }

    #[test]
    fn test_forbidden_error_status() {
        let resp = helpers::forbidden_error("forbidden");
        assert_eq!(resp.status(), http::StatusCode::FORBIDDEN);
    }

    #[test]
    fn test_bad_request_error_status() {
        let resp = helpers::bad_request_error("bad request");
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_internal_error_status() {
        let resp = helpers::internal_error("context", "some error");
        assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_handle_db_option_some() {
        let result: Result<Option<i32>, diesel::result::Error> = Ok(Some(42));
        let value = helpers::handle_db_option(result, "not found", "context");
        assert_eq!(value.unwrap(), 42);
    }

    #[test]
    fn test_handle_db_option_none() {
        let result: Result<Option<i32>, diesel::result::Error> = Ok(None);
        let err = helpers::handle_db_option(result, "not found", "context");
        assert!(err.is_err());
        assert_eq!(err.unwrap_err().status(), http::StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_handle_db_option_error() {
        let result: Result<Option<i32>, diesel::result::Error> =
            Err(diesel::result::Error::NotFound);
        let err = helpers::handle_db_option(result, "not found", "context");
        assert!(err.is_err());
        assert_eq!(err.unwrap_err().status(), http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_handle_db_result_ok() {
        let result: Result<i32, diesel::result::Error> = Ok(42);
        assert_eq!(helpers::handle_db_result(result, "context").unwrap(), 42);
    }

    #[test]
    fn test_handle_db_result_err() {
        let result: Result<i32, diesel::result::Error> = Err(diesel::result::Error::NotFound);
        let err = helpers::handle_db_result(result, "context");
        assert!(err.is_err());
        assert_eq!(err.unwrap_err().status(), http::StatusCode::INTERNAL_SERVER_ERROR);
    }
}
