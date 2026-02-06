use axum::extract::Multipart;
use diesel::{
    ExpressionMethods, JoinOnDsl, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
};
use std::path::Path;
use tokio::fs;

use crate::api::users::User;
use crate::state::GlobalState;

use super::models::{AttachmentResponse, TicketAttachment};
use crate::api::audits::tickets::error::TicketError;

const MAX_FILE_SIZE: usize = 50 * 1024 * 1024; // 50 MB

/// List attachments for a ticket
///
/// # Errors
/// Returns `TicketError` if database error
pub fn list_attachments(
    state: &GlobalState,
    audit_id: &str,
    ticket_id: &str,
) -> Result<Vec<AttachmentResponse>, TicketError> {
    use crate::schema::{ticket_attachments, tickets, users};

    let mut conn = state.get_db_conn()?;

    // Verify ticket belongs to audit
    let _ticket = tickets::table
        .filter(tickets::id.eq(ticket_id))
        .filter(tickets::audit_id.eq(audit_id))
        .select(tickets::id)
        .first::<String>(&mut conn)
        .optional()?
        .ok_or(TicketError::NotFound)?;

    let attachments_with_users = ticket_attachments::table
        .filter(ticket_attachments::ticket_id.eq(ticket_id))
        .inner_join(users::table.on(ticket_attachments::uploaded_by.eq(users::id)))
        .order(ticket_attachments::created_at.desc())
        .select((TicketAttachment::as_select(), User::as_select()))
        .load::<(TicketAttachment, User)>(&mut conn)?;

    Ok(attachments_with_users
        .into_iter()
        .map(|(att, user)| AttachmentResponse::from_attachment_and_user(att, user))
        .collect())
}

/// Upload an attachment
///
/// # Errors
/// Returns `TicketError` if database error, file too large, or storage error
pub async fn upload_attachment(
    state: &GlobalState,
    audit_id: &str,
    ticket_id: &str,
    user_id: &str,
    mut multipart: Multipart,
) -> Result<AttachmentResponse, TicketError> {
    use crate::schema::{ticket_attachments, tickets, users};

    // Verify ticket belongs to audit
    {
        let mut conn = state.get_db_conn()?;
        let _ticket = tickets::table
            .filter(tickets::id.eq(ticket_id))
            .filter(tickets::audit_id.eq(audit_id))
            .select(tickets::id)
            .first::<String>(&mut conn)
            .optional()?
            .ok_or(TicketError::NotFound)?;
    }

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| TicketError::FileStorage(e.to_string()))?
    {
        let name = field.name().unwrap_or("").to_string();
        if name != "file" {
            continue;
        }

        let original_name = field.file_name().unwrap_or("unnamed").to_string();

        let content_type = field
            .content_type()
            .unwrap_or("application/octet-stream")
            .to_string();

        // Reject potentially dangerous file types
        if content_type.starts_with("application/x-executable")
            || content_type.starts_with("application/x-sharedlib")
        {
            return Err(TicketError::InvalidFileType);
        }

        let data = field
            .bytes()
            .await
            .map_err(|e| TicketError::FileStorage(e.to_string()))?;

        if data.len() > MAX_FILE_SIZE {
            return Err(TicketError::FileTooLarge);
        }

        // Create storage directory
        let dir = format!("data/attachments/{audit_id}/{ticket_id}");
        fs::create_dir_all(&dir)
            .await
            .map_err(|e| TicketError::FileStorage(e.to_string()))?;

        let file_id = ulid::Ulid::new().to_string();
        let extension = Path::new(&original_name)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        let stored_name = if extension.is_empty() {
            file_id.clone()
        } else {
            format!("{file_id}.{extension}")
        };

        let file_path = format!("{dir}/{stored_name}");

        fs::write(&file_path, &data)
            .await
            .map_err(|e| TicketError::FileStorage(e.to_string()))?;

        let file_size =
            i64::try_from(data.len()).map_err(|e| TicketError::FileStorage(e.to_string()))?;

        let attachment = TicketAttachment::new(
            ticket_id.to_string(),
            user_id.to_string(),
            original_name,
            file_path,
            file_size,
            content_type,
        );

        let mut conn = state.get_db_conn()?;
        diesel::insert_into(ticket_attachments::table)
            .values(&attachment)
            .execute(&mut conn)?;

        let author = users::table
            .filter(users::id.eq(user_id))
            .select(User::as_select())
            .first::<User>(&mut conn)?;

        return Ok(AttachmentResponse::from_attachment_and_user(
            attachment, author,
        ));
    }

    Err(TicketError::FileStorage("No file field found".to_string()))
}

/// Get attachment details (for download)
///
/// # Errors
/// Returns `TicketError` if not found
pub fn get_attachment(
    state: &GlobalState,
    audit_id: &str,
    ticket_id: &str,
    attachment_id: &str,
) -> Result<TicketAttachment, TicketError> {
    use crate::schema::{ticket_attachments, tickets};

    let mut conn = state.get_db_conn()?;

    // Verify ticket belongs to audit
    let _ticket = tickets::table
        .filter(tickets::id.eq(ticket_id))
        .filter(tickets::audit_id.eq(audit_id))
        .select(tickets::id)
        .first::<String>(&mut conn)
        .optional()?
        .ok_or(TicketError::NotFound)?;

    let attachment = ticket_attachments::table
        .filter(ticket_attachments::id.eq(attachment_id))
        .filter(ticket_attachments::ticket_id.eq(ticket_id))
        .select(TicketAttachment::as_select())
        .first::<TicketAttachment>(&mut conn)
        .optional()?
        .ok_or(TicketError::AttachmentNotFound)?;

    Ok(attachment)
}

/// Delete an attachment
///
/// # Errors
/// Returns `TicketError` if not found or storage error
pub async fn delete_attachment(
    state: &GlobalState,
    audit_id: &str,
    ticket_id: &str,
    attachment_id: &str,
) -> Result<(), TicketError> {
    use crate::schema::ticket_attachments;

    let attachment = get_attachment(state, audit_id, ticket_id, attachment_id)?;

    // Delete file from disk
    if Path::new(&attachment.file_path).exists() {
        fs::remove_file(&attachment.file_path)
            .await
            .map_err(|e| TicketError::FileStorage(e.to_string()))?;
    }

    // Delete from database
    let mut conn = state.get_db_conn()?;
    diesel::delete(ticket_attachments::table.filter(ticket_attachments::id.eq(attachment_id)))
        .execute(&mut conn)?;

    Ok(())
}
