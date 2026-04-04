#[allow(clippy::too_many_lines)]
#[cfg(test)]
mod tests {
    use crate::api::audits::tickets::columns::models::{
        ColumnResponse, CreateColumnPayload, KanbanColumn, ReorderColumnsPayload,
        UpdateColumnPayload,
    };
    use crate::api::audits::tickets::comments::models::{
        CommentResponse, CreateCommentPayload, TicketComment, UpdateCommentPayload,
    };
    use crate::api::audits::tickets::labels::models::{
        CreateLabelPayload, LabelResponse, TicketLabel, TicketLabelAssignment, UpdateLabelPayload,
    };
    use crate::api::audits::tickets::attachments::models::{
        AttachmentResponse, TicketAttachment,
    };
    use crate::api::audits::tickets::models::{
        TicketResponse, TicketUser,
        payload::{CreateTicketPayload, MoveTicketPayload, UpdateTicketPayload},
        priority::TicketPriority,
        ticket::Ticket,
    };
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

    // TicketPriority tests
    #[test]
    fn test_ticket_priority_serde_roundtrip() {
        for (variant, expected) in [
            (TicketPriority::Low, "\"LOW\""),
            (TicketPriority::Medium, "\"MEDIUM\""),
            (TicketPriority::High, "\"HIGH\""),
            (TicketPriority::Critical, "\"CRITICAL\""),
        ] {
            let json = serde_json::to_string(&variant).unwrap();
            assert_eq!(json, expected);
            let deserialized: TicketPriority = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized, variant);
        }
    }

    #[test]
    fn test_ticket_priority_clone_eq_debug() {
        let p = TicketPriority::High;
        let cloned = p.clone();
        assert_eq!(p, cloned);
        assert!(!format!("{p:?}").is_empty());
    }

    // Ticket tests
    #[test]
    fn test_ticket_new_fields() {
        let t = Ticket::new(
            "audit1".into(),
            "col1".into(),
            "My Ticket".into(),
            Some("desc".into()),
            TicketPriority::Medium,
            Some("user1".into()),
            0,
            None,
            None,
            "creator1".into(),
        );
        assert_eq!(t.audit_id, "audit1");
        assert_eq!(t.column_id, "col1");
        assert_eq!(t.title, "My Ticket");
        assert_eq!(t.description.as_deref(), Some("desc"));
        assert_eq!(t.priority, TicketPriority::Medium);
        assert_eq!(t.assignee_id.as_deref(), Some("user1"));
        assert_eq!(t.position, 0);
        assert!(t.due_date.is_none());
        assert!(t.parent_id.is_none());
        assert_eq!(t.created_by, "creator1");
        assert!(!t.id.is_empty());
    }

    #[test]
    fn test_ticket_new_unique_ids() {
        let t1 = Ticket::new(
            "a".into(), "c".into(), "t".into(), None,
            TicketPriority::Low, None, 0, None, None, "u".into(),
        );
        let t2 = Ticket::new(
            "a".into(), "c".into(), "t".into(), None,
            TicketPriority::Low, None, 0, None, None, "u".into(),
        );
        assert_ne!(t1.id, t2.id);
    }

    // TicketUser tests
    #[test]
    fn test_ticket_user_from_user() {
        let user = test_user();
        let tu = TicketUser::from(user.clone());
        assert_eq!(tu.id, user.id);
        assert_eq!(tu.username, "testuser");
        assert_eq!(tu.first_name, "John");
        assert_eq!(tu.last_name, "Doe");
    }

    // TicketResponse tests
    #[test]
    fn test_ticket_response_from_ticket_no_assignee() {
        let ticket = Ticket::new(
            "a1".into(), "c1".into(), "Title".into(), None,
            TicketPriority::Low, None, 0, None, None, test_user().id.clone(),
        );
        let creator = test_user();
        let resp = TicketResponse::from_ticket(ticket.clone(), None, creator, vec![], 0, 0, 0);
        assert_eq!(resp.id, ticket.id);
        assert!(resp.assignee.is_none());
        assert!(resp.labels.is_empty());
        assert_eq!(resp.subtask_count, 0);
    }

    #[test]
    fn test_ticket_response_from_ticket_with_assignee() {
        let ticket = Ticket::new(
            "a1".into(), "c1".into(), "Title".into(), None,
            TicketPriority::High, None, 1, None, None, test_user().id.clone(),
        );
        let creator = test_user();
        let assignee = test_user();
        let resp = TicketResponse::from_ticket(
            ticket, Some(assignee), creator, vec![], 2, 3, 1,
        );
        assert!(resp.assignee.is_some());
        assert_eq!(resp.subtask_count, 2);
        assert_eq!(resp.comment_count, 3);
        assert_eq!(resp.attachment_count, 1);
    }

    // KanbanColumn tests
    #[test]
    fn test_kanban_column_new() {
        let col = KanbanColumn::new("audit1".into(), "To Do".into(), 0);
        assert_eq!(col.audit_id, "audit1");
        assert_eq!(col.name, "To Do");
        assert_eq!(col.position, 0);
        assert!(!col.id.is_empty());
    }

    #[test]
    fn test_kanban_column_unique_ids() {
        let c1 = KanbanColumn::new("a".into(), "n".into(), 0);
        let c2 = KanbanColumn::new("a".into(), "n".into(), 0);
        assert_ne!(c1.id, c2.id);
    }

    #[test]
    fn test_column_response_from() {
        let col = KanbanColumn::new("a1".into(), "Done".into(), 2);
        let resp = ColumnResponse::from(col.clone());
        assert_eq!(resp.id, col.id);
        assert_eq!(resp.name, "Done");
        assert_eq!(resp.position, 2);
    }

    // TicketLabel tests
    #[test]
    fn test_ticket_label_new() {
        let label = TicketLabel::new("a1".into(), "Bug".into(), "#ff0000".into());
        assert_eq!(label.audit_id, "a1");
        assert_eq!(label.name, "Bug");
        assert_eq!(label.color, "#ff0000");
        assert!(!label.id.is_empty());
    }

    #[test]
    fn test_ticket_label_unique_ids() {
        let l1 = TicketLabel::new("a".into(), "n".into(), "c".into());
        let l2 = TicketLabel::new("a".into(), "n".into(), "c".into());
        assert_ne!(l1.id, l2.id);
    }

    #[test]
    fn test_label_response_from() {
        let label = TicketLabel::new("a1".into(), "Feature".into(), "#00ff00".into());
        let resp = LabelResponse::from(label.clone());
        assert_eq!(resp.id, label.id);
        assert_eq!(resp.name, "Feature");
        assert_eq!(resp.color, "#00ff00");
    }

    #[test]
    fn test_ticket_label_assignment_new() {
        let assignment = TicketLabelAssignment::new("t1".into(), "l1".into());
        assert_eq!(assignment.ticket_id, "t1");
        assert_eq!(assignment.label_id, "l1");
    }

    // TicketComment tests
    #[test]
    fn test_ticket_comment_new() {
        let comment = TicketComment::new("t1".into(), "u1".into(), "Hello".into());
        assert_eq!(comment.ticket_id, "t1");
        assert_eq!(comment.author_id, "u1");
        assert_eq!(comment.content, "Hello");
        assert!(!comment.id.is_empty());
    }

    #[test]
    fn test_ticket_comment_unique_ids() {
        let c1 = TicketComment::new("t".into(), "u".into(), "c".into());
        let c2 = TicketComment::new("t".into(), "u".into(), "c".into());
        assert_ne!(c1.id, c2.id);
    }

    #[test]
    fn test_comment_response_from_comment_and_author() {
        let comment = TicketComment::new("t1".into(), "u1".into(), "text".into());
        let author = test_user();
        let resp = CommentResponse::from_comment_and_author(comment.clone(), author);
        assert_eq!(resp.id, comment.id);
        assert_eq!(resp.ticket_id, "t1");
        assert_eq!(resp.content, "text");
        assert_eq!(resp.author.username, "testuser");
    }

    // TicketAttachment tests
    #[test]
    fn test_ticket_attachment_new() {
        let att = TicketAttachment::new(
            "t1".into(), "u1".into(), "file.pdf".into(),
            "/uploads/file.pdf".into(), 1024, "application/pdf".into(),
        );
        assert_eq!(att.ticket_id, "t1");
        assert_eq!(att.uploaded_by, "u1");
        assert_eq!(att.file_name, "file.pdf");
        assert_eq!(att.file_size, 1024);
        assert_eq!(att.mime_type, "application/pdf");
        assert!(!att.id.is_empty());
    }

    #[test]
    fn test_ticket_attachment_unique_ids() {
        let a1 = TicketAttachment::new(
            "t".into(), "u".into(), "f".into(), "p".into(), 0, "m".into(),
        );
        let a2 = TicketAttachment::new(
            "t".into(), "u".into(), "f".into(), "p".into(), 0, "m".into(),
        );
        assert_ne!(a1.id, a2.id);
    }

    #[test]
    fn test_attachment_response_from() {
        let att = TicketAttachment::new(
            "t1".into(), "u1".into(), "img.png".into(),
            "/uploads/img.png".into(), 2048, "image/png".into(),
        );
        let user = test_user();
        let resp = AttachmentResponse::from_attachment_and_user(att.clone(), user);
        assert_eq!(resp.id, att.id);
        assert_eq!(resp.file_name, "img.png");
        assert_eq!(resp.file_size, 2048);
        assert_eq!(resp.mime_type, "image/png");
    }

    // Payload serde tests
    #[test]
    fn test_create_ticket_payload_serde() {
        let json = r#"{"title":"Bug","columnId":"c1"}"#;
        let payload: CreateTicketPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.title, "Bug");
        assert_eq!(payload.column_id, "c1");
        assert!(payload.priority.is_none());
        assert!(payload.assignee_id.is_none());
    }

    #[test]
    fn test_update_ticket_payload_serde() {
        let json = r#"{"title":"Updated"}"#;
        let payload: UpdateTicketPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.title.as_deref(), Some("Updated"));
        assert!(!payload.clear_assignee);
        assert!(!payload.clear_parent);
        assert!(!payload.clear_due_date);
    }

    #[test]
    fn test_move_ticket_payload_serde() {
        let json = r#"{"columnId":"c2","position":3}"#;
        let payload: MoveTicketPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.column_id, "c2");
        assert_eq!(payload.position, 3);
    }

    #[test]
    fn test_create_comment_payload_serde() {
        let json = r#"{"content":"Hello world"}"#;
        let payload: CreateCommentPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.content, "Hello world");
    }

    #[test]
    fn test_update_comment_payload_serde() {
        let json = r#"{"content":"Updated comment"}"#;
        let payload: UpdateCommentPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.content, "Updated comment");
    }

    #[test]
    fn test_create_column_payload_serde() {
        let json = r#"{"name":"In Progress"}"#;
        let payload: CreateColumnPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.name, "In Progress");
    }

    #[test]
    fn test_update_column_payload_serde() {
        let json = r#"{"name":"Done"}"#;
        let payload: UpdateColumnPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.name, "Done");
    }

    #[test]
    fn test_reorder_columns_payload_serde() {
        let json = r#"{"columnIds":["c1","c2","c3"]}"#;
        let payload: ReorderColumnsPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.column_ids, vec!["c1", "c2", "c3"]);
    }

    #[test]
    fn test_create_label_payload_serde() {
        let json = r#"{"name":"Bug"}"#;
        let payload: CreateLabelPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.name, "Bug");
        assert!(payload.color.is_none());
    }

    #[test]
    fn test_create_label_payload_with_color() {
        let json = r##"{"name":"Bug","color":"#ff0000"}"##;
        let payload: CreateLabelPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.color.as_deref(), Some("#ff0000"));
    }

    #[test]
    fn test_update_label_payload_serde() {
        let json = r##"{"name":"Feature","color":"#00ff00"}"##;
        let payload: UpdateLabelPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.name.as_deref(), Some("Feature"));
        assert_eq!(payload.color.as_deref(), Some("#00ff00"));
    }
}
