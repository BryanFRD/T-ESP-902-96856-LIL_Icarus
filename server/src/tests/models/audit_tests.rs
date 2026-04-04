#[cfg(test)]
mod tests {
    use crate::api::audits::models::{
        Audit, AuditClient, AuditResponse, ClientUser, CreateAuditPayload, UpdateAuditPayload,
    };
    use crate::api::audits::messages::models::{
        AuditMessage, AuditMessageAttachment, AuditMessageAttachmentResponse,
        AuditMessageResponse, CreateAuditMessagePayload,
    };
    use crate::api::teams::companies::models::CompanyReturn;
    use crate::api::users::User;

    fn test_user() -> User {
        User::new(
            "testuser".into(),
            "John".into(),
            "Doe".into(),
            "john@example.com".into(),
            "pentester".into(),
            "password123",
        )
    }

    fn test_company() -> CompanyReturn {
        CompanyReturn {
            id: "c1".into(),
            team_id: "t1".into(),
            name: "ACME".into(),
            description: None,
            siret: "12345678901234".into(),
            created_at: "2024-01-01".into(),
            updated_at: "2024-01-01".into(),
        }
    }

    // Audit tests
    #[test]
    fn test_audit_new() {
        let audit = Audit::new(
            "Security Audit".into(),
            Some("desc".into()),
            "team1".into(),
            "company1".into(),
            None,
        );
        assert_eq!(audit.name, "Security Audit");
        assert_eq!(audit.description.as_deref(), Some("desc"));
        assert_eq!(audit.team_id, "team1");
        assert_eq!(audit.company_id, "company1");
        assert!(!audit.is_archived);
        assert!(audit.end_date.is_none());
        assert!(!audit.id.is_empty());
    }

    #[test]
    fn test_audit_unique_ids() {
        let a1 = Audit::new("a".into(), None, "t".into(), "c".into(), None);
        let a2 = Audit::new("a".into(), None, "t".into(), "c".into(), None);
        assert_ne!(a1.id, a2.id);
    }

    #[test]
    fn test_audit_should_auto_archive_no_end_date() {
        let audit = Audit::new("a".into(), None, "t".into(), "c".into(), None);
        assert!(!audit.should_auto_archive());
    }

    #[test]
    fn test_audit_should_auto_archive_future_end_date() {
        let future = chrono::Utc::now().naive_utc() + chrono::Duration::days(30);
        let audit = Audit::new("a".into(), None, "t".into(), "c".into(), Some(future));
        assert!(!audit.should_auto_archive());
    }

    #[test]
    fn test_audit_should_auto_archive_past_end_date() {
        let past = chrono::Utc::now().naive_utc() - chrono::Duration::days(1);
        let mut audit = Audit::new("a".into(), None, "t".into(), "c".into(), Some(past));
        audit.end_date = Some(past);
        assert!(audit.should_auto_archive());
    }

    #[test]
    fn test_audit_should_auto_archive_already_archived() {
        let past = chrono::Utc::now().naive_utc() - chrono::Duration::days(1);
        let mut audit = Audit::new("a".into(), None, "t".into(), "c".into(), Some(past));
        audit.is_archived = true;
        assert!(!audit.should_auto_archive());
    }

    // AuditClient tests
    #[test]
    fn test_audit_client_new() {
        let ac = AuditClient::new("audit1".into(), "user1".into());
        assert_eq!(ac.audit_id, "audit1");
        assert_eq!(ac.user_id, "user1");
    }

    // ClientUser tests
    #[test]
    fn test_client_user_from_user() {
        let user = test_user();
        let cu = ClientUser::from(user.clone());
        assert_eq!(cu.id, user.id);
        assert_eq!(cu.username, "testuser");
        assert_eq!(cu.email, "john@example.com");
    }

    // AuditResponse tests
    #[test]
    fn test_audit_response_from_audit_and_clients() {
        let audit = Audit::new("Audit".into(), None, "t1".into(), "c1".into(), None);
        let clients = vec![test_user()];
        let resp = AuditResponse::from_audit_and_clients(audit.clone(), test_company(), clients);
        assert_eq!(resp.id, audit.id);
        assert_eq!(resp.name, "Audit");
        assert_eq!(resp.clients.len(), 1);
        assert!(!resp.is_archived);
    }

    #[test]
    fn test_audit_response_no_clients() {
        let audit = Audit::new("Audit".into(), None, "t1".into(), "c1".into(), None);
        let resp = AuditResponse::from_audit_and_clients(audit, test_company(), vec![]);
        assert!(resp.clients.is_empty());
    }

    // Payload serde tests
    #[test]
    fn test_create_audit_payload_serde() {
        let json = r#"{"name":"Audit Q1","teamId":"t1","companyId":"c1","endDate":null}"#;
        let payload: CreateAuditPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.name, "Audit Q1");
        assert_eq!(payload.team_id, "t1");
        assert_eq!(payload.company_id, "c1");
        assert!(payload.end_date.is_none());
    }

    #[test]
    fn test_update_audit_payload_serde() {
        let json = r#"{"name":"Updated","endDate":null}"#;
        let payload: UpdateAuditPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.name.as_deref(), Some("Updated"));
        assert!(!payload.clear_end_date);
    }

    // AuditMessage tests
    #[test]
    fn test_audit_message_new() {
        let msg = AuditMessage::new(
            "a1".into(),
            "u1".into(),
            "encrypted_content".into(),
            vec![1, 2, 3],
        );
        assert_eq!(msg.audit_id, "a1");
        assert_eq!(msg.author_id, "u1");
        assert_eq!(msg.content, "encrypted_content");
        assert_eq!(msg.content_nonce, vec![1, 2, 3]);
        assert!(!msg.id.is_empty());
    }

    #[test]
    fn test_audit_message_unique_ids() {
        let m1 = AuditMessage::new("a".into(), "u".into(), "c".into(), vec![]);
        let m2 = AuditMessage::new("a".into(), "u".into(), "c".into(), vec![]);
        assert_ne!(m1.id, m2.id);
    }

    // AuditMessageAttachment tests
    #[test]
    fn test_audit_message_attachment_new() {
        let att = AuditMessageAttachment::new(
            "m1".into(), "u1".into(), "doc.pdf".into(),
            "/uploads/doc.pdf".into(), 4096, "application/pdf".into(),
        );
        assert_eq!(att.message_id, "m1");
        assert_eq!(att.uploaded_by, "u1");
        assert_eq!(att.file_name, "doc.pdf");
        assert_eq!(att.file_size, 4096);
        assert!(!att.id.is_empty());
    }

    // AuditMessageResponse tests
    #[test]
    fn test_audit_message_response_from() {
        let msg = AuditMessage::new("a1".into(), "u1".into(), "enc".into(), vec![]);
        let author = test_user();
        let resp = AuditMessageResponse::from_message_and_author(
            msg.clone(), author, "decrypted content".into(),
        );
        assert_eq!(resp.id, msg.id);
        assert_eq!(resp.audit_id, "a1");
        assert_eq!(resp.content, "decrypted content");
        assert_eq!(resp.author.username, "testuser");
    }

    // AuditMessageAttachmentResponse tests
    #[test]
    fn test_audit_message_attachment_response_from() {
        let att = AuditMessageAttachment::new(
            "m1".into(), "u1".into(), "file.txt".into(),
            "/path".into(), 100, "text/plain".into(),
        );
        let user = test_user();
        let resp = AuditMessageAttachmentResponse::from_attachment_and_user(att.clone(), user);
        assert_eq!(resp.id, att.id);
        assert_eq!(resp.message_id, "m1");
        assert_eq!(resp.file_name, "file.txt");
    }

    // CreateAuditMessagePayload serde
    #[test]
    fn test_create_audit_message_payload_serde() {
        let json = r#"{"content":"Hello"}"#;
        let payload: CreateAuditMessagePayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.content, "Hello");
    }
}
