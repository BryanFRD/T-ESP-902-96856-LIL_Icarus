use axum::{
    Router,
    extract::Multipart,
    middleware::{from_fn, from_fn_with_state},
    routing::{delete, get, post},
};
use diesel::{
    ExpressionMethods, JoinOnDsl, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
};
use std::path::Path;
use tokio::fs;
use tracing::error;

use crate::api::audits::Audit;
use crate::api::audits::messages::endpoints;
use crate::api::audits::messages::error::AuditMessageError;
use crate::api::audits::messages::models::{
    AuditMessage, AuditMessageAttachment, AuditMessageAttachmentResponse, AuditMessageResponse,
    CreateAuditMessagePayload, PaginatedAuditMessageResponse,
};
use crate::api::users::User;
use crate::middlewares::{audit_step_up_middleware, auth};
use crate::state::GlobalState;
use crate::utils::event_encryption::{decrypt_json, encrypt_json};

const MAX_FILE_SIZE: usize = 25 * 1024 * 1024; // 25 MB

pub fn get_routes(state: &GlobalState) -> Router<GlobalState> {
    Router::new()
        .route("/{audit_id}/messages", get(endpoints::list_messages))
        .route("/{audit_id}/messages", post(endpoints::create_message))
        .route(
            "/{audit_id}/messages/{message_id}/attachments",
            get(endpoints::list_attachments),
        )
        .route(
            "/{audit_id}/messages/{message_id}/attachments",
            post(endpoints::upload_attachment),
        )
        .route(
            "/{audit_id}/messages/{message_id}/attachments/{attachment_id}",
            get(endpoints::download_attachment),
        )
        .route(
            "/{audit_id}/messages/{message_id}/attachments/{attachment_id}",
            delete(endpoints::delete_attachment),
        )
        .layer(from_fn_with_state(state.clone(), audit_step_up_middleware))
        .layer(from_fn(auth::middleware))
}

pub fn require_audit_access(
    state: &GlobalState,
    audit_id: &str,
    user_id: &str,
    is_admin: bool,
) -> Result<Audit, AuditMessageError> {
    use crate::schema::{audits, audits_clients, teams_members};

    let mut conn = state.get_db_conn()?;
    let audit = audits::table
        .filter(audits::id.eq(audit_id))
        .select(Audit::as_select())
        .first::<Audit>(&mut conn)
        .optional()?
        .ok_or(AuditMessageError::Audit(
            crate::api::audits::AuditError::NotFound,
        ))?;

    if audit.is_archived {
        return Err(AuditMessageError::AuditArchived);
    }

    if is_admin {
        return Ok(audit);
    }

    let is_team_member = teams_members::table
        .filter(teams_members::team_id.eq(&audit.team_id))
        .filter(teams_members::user_id.eq(user_id))
        .count()
        .get_result::<i64>(&mut conn)?
        > 0;

    if is_team_member {
        return Ok(audit);
    }

    let is_audit_client = audits_clients::table
        .filter(audits_clients::audit_id.eq(audit_id))
        .filter(audits_clients::user_id.eq(user_id))
        .count()
        .get_result::<i64>(&mut conn)?
        > 0;

    if is_audit_client {
        return Ok(audit);
    }

    Err(AuditMessageError::NotAuthorized)
}

fn decrypt_content(state: &GlobalState, message: &AuditMessage) -> String {
    match decrypt_json(
        &state.event_encryption_key,
        &message.content,
        &message.content_nonce,
    ) {
        Ok(value) => value
            .get("content")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        Err(e) => {
            error!(message_id = %message.id, error = %e, "Failed to decrypt audit message");
            "[decryption_failed]".to_string()
        }
    }
}

pub fn list_messages(
    state: &GlobalState,
    audit_id: &str,
    user_id: &str,
    is_admin: bool,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<PaginatedAuditMessageResponse, AuditMessageError> {
    use crate::schema::{audit_messages, users};

    let _audit = require_audit_access(state, audit_id, user_id, is_admin)?;

    let mut conn = state.get_db_conn()?;
    let limit_val = limit.unwrap_or(100).min(500);
    let offset_val = offset.unwrap_or(0);

    let total = audit_messages::table
        .filter(audit_messages::audit_id.eq(audit_id))
        .count()
        .get_result::<i64>(&mut conn)?;

    let messages_with_authors = audit_messages::table
        .filter(audit_messages::audit_id.eq(audit_id))
        .inner_join(users::table.on(audit_messages::author_id.eq(users::id)))
        .order(audit_messages::created_at.asc())
        .limit(limit_val)
        .offset(offset_val)
        .select((AuditMessage::as_select(), User::as_select()))
        .load::<(AuditMessage, User)>(&mut conn)?;

    let data = messages_with_authors
        .into_iter()
        .map(|(message, author)| {
            let content = decrypt_content(state, &message);
            AuditMessageResponse::from_message_and_author(message, author, content)
        })
        .collect();

    Ok(PaginatedAuditMessageResponse {
        data,
        total,
        limit: limit_val,
        offset: offset_val,
    })
}

pub fn create_message(
    state: &GlobalState,
    audit_id: &str,
    user_id: &str,
    payload: &CreateAuditMessagePayload,
    is_admin: bool,
) -> Result<AuditMessageResponse, AuditMessageError> {
    use crate::schema::{audit_messages, users};

    let _audit = require_audit_access(state, audit_id, user_id, is_admin)?;

    let (ciphertext, nonce) = encrypt_json(
        &state.event_encryption_key,
        &serde_json::json!({ "content": payload.content }),
    )?;

    let message = AuditMessage::new(audit_id.to_string(), user_id.to_string(), ciphertext, nonce);

    let mut conn = state.get_db_conn()?;
    diesel::insert_into(audit_messages::table)
        .values(&message)
        .execute(&mut conn)?;

    let author = users::table
        .filter(users::id.eq(user_id))
        .select(User::as_select())
        .first::<User>(&mut conn)?;

    let content = decrypt_content(state, &message);
    Ok(AuditMessageResponse::from_message_and_author(
        message, author, content,
    ))
}

pub fn list_attachments(
    state: &GlobalState,
    audit_id: &str,
    message_id: &str,
    user_id: &str,
    is_admin: bool,
) -> Result<Vec<AuditMessageAttachmentResponse>, AuditMessageError> {
    use crate::schema::{audit_message_attachments, audit_messages, users};

    let _audit = require_audit_access(state, audit_id, user_id, is_admin)?;

    let mut conn = state.get_db_conn()?;
    let message = audit_messages::table
        .filter(audit_messages::id.eq(message_id))
        .filter(audit_messages::audit_id.eq(audit_id))
        .select(AuditMessage::as_select())
        .first::<AuditMessage>(&mut conn)
        .optional()?
        .ok_or(AuditMessageError::MessageNotFound)?;

    let attachments_with_users = audit_message_attachments::table
        .filter(audit_message_attachments::message_id.eq(&message.id))
        .inner_join(users::table.on(audit_message_attachments::uploaded_by.eq(users::id)))
        .order(audit_message_attachments::created_at.asc())
        .select((AuditMessageAttachment::as_select(), User::as_select()))
        .load::<(AuditMessageAttachment, User)>(&mut conn)?;

    Ok(attachments_with_users
        .into_iter()
        .map(|(att, user)| AuditMessageAttachmentResponse::from_attachment_and_user(att, user))
        .collect())
}

pub async fn upload_attachment(
    state: &GlobalState,
    audit_id: &str,
    message_id: &str,
    user_id: &str,
    mut multipart: Multipart,
    is_admin: bool,
) -> Result<AuditMessageAttachmentResponse, AuditMessageError> {
    use crate::schema::{audit_message_attachments, audit_messages, users};

    let _audit = require_audit_access(state, audit_id, user_id, is_admin)?;

    {
        let mut conn = state.get_db_conn()?;
        let _message = audit_messages::table
            .filter(audit_messages::id.eq(message_id))
            .filter(audit_messages::audit_id.eq(audit_id))
            .select(AuditMessage::as_select())
            .first::<AuditMessage>(&mut conn)
            .optional()?
            .ok_or(AuditMessageError::MessageNotFound)?;
    }

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AuditMessageError::FileStorage(e.to_string()))?
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

        let data = field
            .bytes()
            .await
            .map_err(|e| AuditMessageError::FileStorage(e.to_string()))?;

        if data.len() > MAX_FILE_SIZE {
            return Err(AuditMessageError::FileTooLarge);
        }

        let dir = format!("data/audit_messages/{audit_id}/{message_id}");
        fs::create_dir_all(&dir)
            .await
            .map_err(|e| AuditMessageError::FileStorage(e.to_string()))?;

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
            .map_err(|e| AuditMessageError::FileStorage(e.to_string()))?;

        let file_size =
            i64::try_from(data.len()).map_err(|e| AuditMessageError::FileStorage(e.to_string()))?;

        let attachment = AuditMessageAttachment::new(
            message_id.to_string(),
            user_id.to_string(),
            original_name,
            file_path,
            file_size,
            content_type,
        );

        let mut conn = state.get_db_conn()?;
        diesel::insert_into(audit_message_attachments::table)
            .values(&attachment)
            .execute(&mut conn)?;

        let author = users::table
            .filter(users::id.eq(user_id))
            .select(User::as_select())
            .first::<User>(&mut conn)?;

        return Ok(AuditMessageAttachmentResponse::from_attachment_and_user(
            attachment, author,
        ));
    }

    Err(AuditMessageError::FileStorage(
        "No file field found".to_string(),
    ))
}

pub fn get_attachment(
    state: &GlobalState,
    audit_id: &str,
    message_id: &str,
    attachment_id: &str,
    user_id: &str,
    is_admin: bool,
) -> Result<AuditMessageAttachment, AuditMessageError> {
    use crate::schema::{audit_message_attachments, audit_messages};

    let _audit = require_audit_access(state, audit_id, user_id, is_admin)?;

    let mut conn = state.get_db_conn()?;
    let _message = audit_messages::table
        .filter(audit_messages::id.eq(message_id))
        .filter(audit_messages::audit_id.eq(audit_id))
        .select(AuditMessage::as_select())
        .first::<AuditMessage>(&mut conn)
        .optional()?
        .ok_or(AuditMessageError::MessageNotFound)?;

    let attachment = audit_message_attachments::table
        .filter(audit_message_attachments::id.eq(attachment_id))
        .filter(audit_message_attachments::message_id.eq(message_id))
        .select(AuditMessageAttachment::as_select())
        .first::<AuditMessageAttachment>(&mut conn)
        .optional()?
        .ok_or(AuditMessageError::AttachmentNotFound)?;

    Ok(attachment)
}

pub async fn delete_attachment(
    state: &GlobalState,
    audit_id: &str,
    message_id: &str,
    attachment_id: &str,
    user_id: &str,
    is_admin: bool,
) -> Result<(), AuditMessageError> {
    use crate::schema::audit_message_attachments;

    let attachment = get_attachment(
        state,
        audit_id,
        message_id,
        attachment_id,
        user_id,
        is_admin,
    )?;

    if Path::new(&attachment.file_path).exists() {
        fs::remove_file(&attachment.file_path)
            .await
            .map_err(|e| AuditMessageError::FileStorage(e.to_string()))?;
    }

    let mut conn = state.get_db_conn()?;
    diesel::delete(
        audit_message_attachments::table.filter(audit_message_attachments::id.eq(attachment_id)),
    )
    .execute(&mut conn)?;

    Ok(())
}
