#[cfg(test)]
mod tests {
    use crate::api::teams::{
        Company, CompanyReturn, CreateCompanyPayload, UpdateCompanyPayload,
        TeamInvitation, InvitationReturn,
        TeamError,
    };
    use crate::api::users::UserError;

    // ===== Company tests =====

    #[test]
    fn test_company_new() {
        let c = Company::new(
            "team1".into(),
            "ACME".into(),
            Some("A company".into()),
            "12345678901234".into(),
        );
        assert_eq!(c.team_id, "team1");
        assert_eq!(c.name, "ACME");
        assert_eq!(c.description.as_deref(), Some("A company"));
        assert_eq!(c.siret, "12345678901234");
        assert!(!c.id.is_empty());
    }

    #[test]
    fn test_company_unique_ids() {
        let c1 = Company::new("t".into(), "n".into(), None, "s".into());
        let c2 = Company::new("t".into(), "n".into(), None, "s".into());
        assert_ne!(c1.id, c2.id);
    }

    #[test]
    fn test_company_return_from_company() {
        let c = Company::new("t1".into(), "Name".into(), None, "siret".into());
        let ret = CompanyReturn::from(c.clone());
        assert_eq!(ret.id, c.id);
        assert_eq!(ret.team_id, "t1");
        assert_eq!(ret.name, "Name");
        assert!(ret.description.is_none());
        assert_eq!(ret.siret, "siret");
    }

    #[test]
    fn test_create_company_payload_serde() {
        let json = r#"{"name":"Corp","siret":"123"}"#;
        let p: CreateCompanyPayload = serde_json::from_str(json).unwrap();
        assert_eq!(p.name, "Corp");
        assert!(p.description.is_none());
        assert_eq!(p.siret, "123");
    }

    #[test]
    fn test_create_company_payload_with_description() {
        let json = r#"{"name":"Corp","description":"Desc","siret":"123"}"#;
        let p: CreateCompanyPayload = serde_json::from_str(json).unwrap();
        assert_eq!(p.description.as_deref(), Some("Desc"));
    }

    #[test]
    fn test_update_company_payload_serde() {
        let json = r#"{"name":"New"}"#;
        let p: UpdateCompanyPayload = serde_json::from_str(json).unwrap();
        assert_eq!(p.name.as_deref(), Some("New"));
        assert!(p.description.is_none());
        assert!(p.siret.is_none());
    }

    // ===== TeamInvitation tests =====

    #[test]
    fn test_team_invitation_new() {
        let inv = TeamInvitation::new("team1".into(), "user1".into(), 5, None);
        assert_eq!(inv.team_id, "team1");
        assert_eq!(inv.created_by, "user1");
        assert_eq!(inv.max_uses, 5);
        assert_eq!(inv.used_count, 0);
        assert!(!inv.is_canceled);
        assert!(inv.expires_at.is_none());
        assert!(!inv.id.is_empty());
        assert!(!inv.token.is_empty());
    }

    #[test]
    fn test_team_invitation_unique_ids() {
        let i1 = TeamInvitation::new("t".into(), "u".into(), 1, None);
        let i2 = TeamInvitation::new("t".into(), "u".into(), 1, None);
        assert_ne!(i1.id, i2.id);
        assert_ne!(i1.token, i2.token);
    }

    #[test]
    fn test_team_invitation_with_expiry() {
        let exp = chrono::Utc::now().naive_utc() + chrono::Duration::hours(24);
        let inv = TeamInvitation::new("t".into(), "u".into(), 10, Some(exp));
        assert!(inv.expires_at.is_some());
    }

    #[test]
    fn test_invitation_return_from_invitation() {
        let inv = TeamInvitation::new("t1".into(), "u1".into(), 3, None);
        let ret = InvitationReturn::from(inv.clone());
        assert_eq!(ret.id, inv.id);
        assert_eq!(ret.team_id, "t1");
        assert_eq!(ret.token, inv.token);
        assert_eq!(ret.max_uses, 3);
        assert_eq!(ret.used_count, 0);
        assert!(!ret.is_canceled);
    }

    // ===== Error Display tests =====

    #[test]
    fn test_user_error_display() {
        assert_eq!(UserError::UserNotFound.to_string(), "User not found");
        assert_eq!(UserError::AvatarNotFound.to_string(), "Avatar not found");
        assert_eq!(
            UserError::UpdatePayloadError("bad field".into()).to_string(),
            "bad field"
        );
        assert_eq!(
            UserError::MultipartError("invalid".into()).to_string(),
            "invalid"
        );
    }

    #[test]
    fn test_team_error_display() {
        let err = TeamError::User(UserError::UserNotFound);
        assert_eq!(err.to_string(), "Error while handling user: User not found");
    }
}
