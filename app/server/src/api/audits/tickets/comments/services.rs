use diesel::{
    ExpressionMethods, JoinOnDsl, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
};

use crate::api::users::User;
use crate::state::GlobalState;

use super::models::{CommentResponse, CreateCommentPayload, TicketComment, UpdateCommentPayload};
use crate::api::audits::tickets::error::TicketError;

/// List comments for a ticket
///
/// # Errors
/// Returns `TicketError` if database error
pub fn list_comments(
    state: &GlobalState,
    audit_id: &str,
    ticket_id: &str,
) -> Result<Vec<CommentResponse>, TicketError> {
    use crate::schema::{ticket_comments, tickets, users};

    let mut conn = state.get_db_conn()?;

    // Verify ticket belongs to audit
    let _ticket = tickets::table
        .filter(tickets::id.eq(ticket_id))
        .filter(tickets::audit_id.eq(audit_id))
        .select(tickets::id)
        .first::<String>(&mut conn)
        .optional()?
        .ok_or(TicketError::NotFound)?;

    let comments_with_authors = ticket_comments::table
        .filter(ticket_comments::ticket_id.eq(ticket_id))
        .inner_join(users::table.on(ticket_comments::author_id.eq(users::id)))
        .order(ticket_comments::created_at.asc())
        .select((TicketComment::as_select(), User::as_select()))
        .load::<(TicketComment, User)>(&mut conn)?;

    Ok(comments_with_authors
        .into_iter()
        .map(|(comment, author)| CommentResponse::from_comment_and_author(comment, author))
        .collect())
}

/// Create a comment on a ticket
///
/// # Errors
/// Returns `TicketError` if database error or ticket not found
pub fn create_comment(
    state: &GlobalState,
    audit_id: &str,
    ticket_id: &str,
    user_id: &str,
    payload: &CreateCommentPayload,
) -> Result<CommentResponse, TicketError> {
    use crate::schema::{ticket_comments, tickets, users};

    let mut conn = state.get_db_conn()?;

    // Verify ticket belongs to audit
    let _ticket = tickets::table
        .filter(tickets::id.eq(ticket_id))
        .filter(tickets::audit_id.eq(audit_id))
        .select(tickets::id)
        .first::<String>(&mut conn)
        .optional()?
        .ok_or(TicketError::NotFound)?;

    let comment = TicketComment::new(
        ticket_id.to_string(),
        user_id.to_string(),
        payload.content.clone(),
    );

    diesel::insert_into(ticket_comments::table)
        .values(&comment)
        .execute(&mut conn)?;

    let author = users::table
        .filter(users::id.eq(user_id))
        .select(User::as_select())
        .first::<User>(&mut conn)?;

    Ok(CommentResponse::from_comment_and_author(comment, author))
}

/// Update a comment (author only)
///
/// # Errors
/// Returns `TicketError` if database error, not found, or not author
pub fn update_comment(
    state: &GlobalState,
    audit_id: &str,
    ticket_id: &str,
    comment_id: &str,
    user_id: &str,
    payload: &UpdateCommentPayload,
) -> Result<CommentResponse, TicketError> {
    use crate::schema::{ticket_comments, tickets, users};

    let mut conn = state.get_db_conn()?;

    // Verify ticket belongs to audit
    let _ticket = tickets::table
        .filter(tickets::id.eq(ticket_id))
        .filter(tickets::audit_id.eq(audit_id))
        .select(tickets::id)
        .first::<String>(&mut conn)
        .optional()?
        .ok_or(TicketError::NotFound)?;

    let comment = ticket_comments::table
        .filter(ticket_comments::id.eq(comment_id))
        .filter(ticket_comments::ticket_id.eq(ticket_id))
        .select(TicketComment::as_select())
        .first::<TicketComment>(&mut conn)
        .optional()?
        .ok_or(TicketError::CommentNotFound)?;

    if comment.author_id != user_id {
        return Err(TicketError::NotCommentAuthor);
    }

    diesel::update(ticket_comments::table.filter(ticket_comments::id.eq(comment_id)))
        .set(ticket_comments::content.eq(&payload.content))
        .execute(&mut conn)?;

    let author = users::table
        .filter(users::id.eq(&comment.author_id))
        .select(User::as_select())
        .first::<User>(&mut conn)?;

    let updated = TicketComment {
        content: payload.content.clone(),
        ..comment
    };

    Ok(CommentResponse::from_comment_and_author(updated, author))
}

/// Delete a comment
///
/// # Errors
/// Returns `TicketError` if database error or not found
pub fn delete_comment(
    state: &GlobalState,
    audit_id: &str,
    ticket_id: &str,
    comment_id: &str,
) -> Result<(), TicketError> {
    use crate::schema::{ticket_comments, tickets};

    let mut conn = state.get_db_conn()?;

    // Verify ticket belongs to audit
    let _ticket = tickets::table
        .filter(tickets::id.eq(ticket_id))
        .filter(tickets::audit_id.eq(audit_id))
        .select(tickets::id)
        .first::<String>(&mut conn)
        .optional()?
        .ok_or(TicketError::NotFound)?;

    let _comment = ticket_comments::table
        .filter(ticket_comments::id.eq(comment_id))
        .filter(ticket_comments::ticket_id.eq(ticket_id))
        .select(ticket_comments::id)
        .first::<String>(&mut conn)
        .optional()?
        .ok_or(TicketError::CommentNotFound)?;

    diesel::delete(ticket_comments::table.filter(ticket_comments::id.eq(comment_id)))
        .execute(&mut conn)?;

    Ok(())
}
