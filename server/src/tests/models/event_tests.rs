#[allow(clippy::too_many_lines)]
#[cfg(test)]
mod tests {
    use crate::api::events::models::{
        AuditLogFilters, EventAction, EventChanges, EventFilters, EventResponse, NewEvent,
        ResourceType,
    };

    // EventAction Display
    #[test]
    fn test_event_action_display() {
        let cases = vec![
            (EventAction::Read, "read"),
            (EventAction::Create, "create"),
            (EventAction::Update, "update"),
            (EventAction::Delete, "delete"),
            (EventAction::Login, "login"),
            (EventAction::Logout, "logout"),
            (EventAction::Register, "register"),
            (EventAction::PasswordReset, "password_reset"),
            (EventAction::EmailConfirm, "email_confirm"),
            (EventAction::InvitationCreate, "invitation_create"),
            (EventAction::InvitationConsume, "invitation_consume"),
            (EventAction::InvitationCancel, "invitation_cancel"),
            (EventAction::AvatarUpload, "avatar_upload"),
            (EventAction::AvatarDelete, "avatar_delete"),
            (EventAction::AgentConnect, "agent_connect"),
            (EventAction::AgentDisconnect, "agent_disconnect"),
            (EventAction::AgentCommand, "agent_command"),
            (EventAction::AgentShutdown, "agent_shutdown"),
        ];
        for (action, expected) in cases {
            assert_eq!(action.to_string(), expected);
        }
    }

    // ResourceType Display
    #[test]
    fn test_resource_type_display() {
        let cases = vec![
            (ResourceType::User, "user"),
            (ResourceType::Team, "team"),
            (ResourceType::TeamMember, "team_member"),
            (ResourceType::Company, "company"),
            (ResourceType::Invitation, "invitation"),
            (ResourceType::Auth, "auth"),
            (ResourceType::Avatar, "avatar"),
            (ResourceType::Audit, "audit"),
            (ResourceType::AuditClient, "audit_client"),
            (ResourceType::AuditInvitation, "audit_invitation"),
            (ResourceType::Ticket, "ticket"),
            (ResourceType::TicketComment, "ticket_comment"),
            (ResourceType::TicketAttachment, "ticket_attachment"),
            (ResourceType::KanbanColumn, "kanban_column"),
            (ResourceType::TicketLabel, "ticket_label"),
            (ResourceType::Agent, "agent"),
        ];
        for (rt, expected) in cases {
            assert_eq!(rt.to_string(), expected);
        }
    }

    // EventAction serde
    #[test]
    fn test_event_action_serde_roundtrip() {
        let action = EventAction::Login;
        let json = serde_json::to_string(&action).unwrap();
        let deserialized: EventAction = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, action);
    }

    #[test]
    fn test_resource_type_serde_roundtrip() {
        let rt = ResourceType::Ticket;
        let json = serde_json::to_string(&rt).unwrap();
        let deserialized: ResourceType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, rt);
    }

    // NewEvent tests
    #[test]
    fn test_new_event_fields() {
        let event = NewEvent::new(
            Some("user1".into()),
            EventAction::Create,
            ResourceType::Ticket,
            Some("ticket1".into()),
            "POST".into(),
            "/v1/audits/a1/tickets".into(),
        );
        assert!(!event.id.is_empty());
        assert_eq!(event.user_id.as_deref(), Some("user1"));
        assert_eq!(event.action, EventAction::Create);
        assert_eq!(event.resource_type, ResourceType::Ticket);
        assert_eq!(event.resource_id.as_deref(), Some("ticket1"));
        assert_eq!(event.request_method, "POST");
        assert!(event.ip_address.is_none());
        assert!(event.user_agent.is_none());
        assert!(event.metadata.is_none());
        assert!(event.changes.is_none());
    }

    #[test]
    fn test_new_event_unique_ids() {
        let e1 = NewEvent::new(None, EventAction::Read, ResourceType::User, None, "GET".into(), "/".into());
        let e2 = NewEvent::new(None, EventAction::Read, ResourceType::User, None, "GET".into(), "/".into());
        assert_ne!(e1.id, e2.id);
    }

    #[test]
    fn test_new_event_builder_with_ip() {
        let event = NewEvent::new(None, EventAction::Read, ResourceType::User, None, "GET".into(), "/".into())
            .with_ip_address(Some("1.2.3.4".into()));
        assert_eq!(event.ip_address.as_deref(), Some("1.2.3.4"));
    }

    #[test]
    fn test_new_event_builder_with_user_agent() {
        let event = NewEvent::new(None, EventAction::Read, ResourceType::User, None, "GET".into(), "/".into())
            .with_user_agent(Some("Mozilla/5.0".into()));
        assert_eq!(event.user_agent.as_deref(), Some("Mozilla/5.0"));
    }

    #[test]
    fn test_new_event_builder_with_metadata() {
        let meta = serde_json::json!({"key": "value"});
        let event = NewEvent::new(None, EventAction::Read, ResourceType::User, None, "GET".into(), "/".into())
            .with_metadata(meta.clone());
        assert_eq!(event.metadata, Some(meta));
    }

    #[test]
    fn test_new_event_builder_with_changes() {
        let changes = EventChanges::new(
            Some(serde_json::json!({"name": "old"})),
            Some(serde_json::json!({"name": "new"})),
        );
        let event = NewEvent::new(None, EventAction::Update, ResourceType::User, None, "PATCH".into(), "/".into())
            .with_changes(changes);
        assert!(event.changes.is_some());
    }

    #[test]
    fn test_new_event_builder_with_nonces() {
        let event = NewEvent::new(None, EventAction::Read, ResourceType::User, None, "GET".into(), "/".into())
            .with_metadata_nonce(Some(vec![1, 2, 3]))
            .with_changes_nonce(Some(vec![4, 5, 6]));
        assert_eq!(event.metadata_nonce, Some(vec![1, 2, 3]));
        assert_eq!(event.changes_nonce, Some(vec![4, 5, 6]));
    }

    // EventChanges tests
    #[test]
    fn test_event_changes_new() {
        let changes = EventChanges::new(None, None);
        assert!(changes.before.is_none());
        assert!(changes.after.is_none());
    }

    #[test]
    fn test_event_changes_with_values() {
        let changes = EventChanges::new(
            Some(serde_json::json!({"a": 1})),
            Some(serde_json::json!({"a": 2})),
        );
        assert!(changes.before.is_some());
        assert!(changes.after.is_some());
    }

    // EventResponse tests
    #[test]
    fn test_event_response_from_event() {
        use crate::api::events::models::Event;
        let now = chrono::Utc::now().naive_utc();
        let event = Event {
            id: "e1".into(),
            user_id: Some("u1".into()),
            action: EventAction::Login,
            resource_type: ResourceType::Auth,
            resource_id: None,
            request_method: "POST".into(),
            request_path: "/v1/auth/login".into(),
            ip_address: None,
            user_agent: None,
            metadata: None,
            changes: None,
            metadata_nonce: None,
            changes_nonce: None,
            created_at: now,
            expires_at: now,
        };
        let resp = EventResponse::from(event);
        assert_eq!(resp.id, "e1");
        assert_eq!(resp.user_id.as_deref(), Some("u1"));
        assert_eq!(resp.action, EventAction::Login);
        assert_eq!(resp.resource_type, ResourceType::Auth);
    }

    // EventFilters tests
    #[test]
    fn test_event_filters_default_anonymized() {
        let json = "{}";
        let filters: EventFilters = serde_json::from_str(json).unwrap();
        assert!(filters.anonymized);
        assert!(filters.user_id.is_none());
        assert!(filters.limit.is_none());
    }

    #[test]
    fn test_event_filters_anonymized_false() {
        let json = r#"{"anonymized": false}"#;
        let filters: EventFilters = serde_json::from_str(json).unwrap();
        assert!(!filters.anonymized);
    }

    // AuditLogFilters tests
    #[test]
    fn test_audit_log_filters_serde() {
        let json = r#"{"includeUser": true, "limit": 50}"#;
        let filters: AuditLogFilters = serde_json::from_str(json).unwrap();
        assert_eq!(filters.include_user, Some(true));
        assert_eq!(filters.limit, Some(50));
        assert!(filters.action.is_none());
        assert!(filters.offset.is_none());
    }
}
