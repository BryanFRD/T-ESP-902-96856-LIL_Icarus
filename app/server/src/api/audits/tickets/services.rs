use std::collections::HashMap;

use axum::{
    Router,
    middleware::{from_fn, from_fn_with_state},
    routing::{delete, get, patch, post, put},
};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{
    middlewares::{audit_step_up_middleware, auth},
    state::GlobalState,
};

use super::{
    columns, comments, endpoints,
    error::TicketError,
    labels,
    models::{
        BoardColumn, BoardResponse, CreateTicketPayload, MoveTicketPayload, Ticket, TicketResponse,
        TicketSummary, TicketUser, UpdateTicketPayload,
    },
};
use crate::api::audits::tickets::labels::models::{LabelResponse, TicketLabel};
use crate::api::audits::{Audit, services as audit_services};
use crate::api::teams::services as team_services;
use crate::api::users::User;

/// Check that the audit exists and is not archived.
///
/// # Errors
/// Returns `TicketError::AuditArchived` if the audit is archived.
fn require_audit_not_archived(state: &GlobalState, audit_id: &str) -> Result<Audit, TicketError> {
    let audit = audit_services::get_audit(state, audit_id)?;
    if audit.is_archived {
        return Err(TicketError::AuditArchived);
    }
    Ok(audit)
}

/// Check that the user is a member of the audit's team.
///
/// # Errors
/// Returns `TicketError::MustBeTeamMember` if the user is not a team member.
fn require_team_member(
    state: &GlobalState,
    team_id: &str,
    user_id: &str,
) -> Result<(), TicketError> {
    if team_services::get_team_member(state, team_id, user_id)?.is_none() {
        return Err(TicketError::MustBeTeamMember);
    }
    Ok(())
}

/// Check that the user is an owner or moderator of the audit's team (or admin).
///
/// # Errors
/// Returns `TicketError::MustBeOwnerOrAdmin` if the user is not authorized.
fn require_owner_or_moderator(
    state: &GlobalState,
    team_id: &str,
    user_id: &str,
    is_admin: bool,
) -> Result<(), TicketError> {
    if is_admin {
        return Ok(());
    }
    if !team_services::is_team_owner_or_moderator(state, team_id, user_id)? {
        return Err(TicketError::MustBeOwnerOrAdmin);
    }
    Ok(())
}

/// Get routes for the tickets module (nested under /audits)
pub fn get_routes(state: &GlobalState) -> Router<GlobalState> {
    Router::new()
        // Board
        .route("/{audit_id}/board", get(endpoints::get_board))
        // Columns
        .route("/{audit_id}/columns", get(columns::endpoints::list_columns))
        .route(
            "/{audit_id}/columns",
            post(columns::endpoints::create_column),
        )
        .route(
            "/{audit_id}/columns/reorder",
            put(columns::endpoints::reorder_columns),
        )
        .route(
            "/{audit_id}/columns/{column_id}",
            patch(columns::endpoints::update_column),
        )
        .route(
            "/{audit_id}/columns/{column_id}",
            delete(columns::endpoints::delete_column),
        )
        // Tickets
        .route("/{audit_id}/tickets", get(endpoints::list_tickets))
        .route("/{audit_id}/tickets", post(endpoints::create_ticket))
        .route(
            "/{audit_id}/tickets/{ticket_id}",
            get(endpoints::get_ticket),
        )
        .route(
            "/{audit_id}/tickets/{ticket_id}",
            patch(endpoints::update_ticket),
        )
        .route(
            "/{audit_id}/tickets/{ticket_id}",
            delete(endpoints::delete_ticket),
        )
        .route(
            "/{audit_id}/tickets/{ticket_id}/move",
            put(endpoints::move_ticket),
        )
        .route(
            "/{audit_id}/tickets/{ticket_id}/subtasks",
            get(endpoints::list_subtasks),
        )
        // Labels
        .route("/{audit_id}/labels", get(labels::endpoints::list_labels))
        .route("/{audit_id}/labels", post(labels::endpoints::create_label))
        .route(
            "/{audit_id}/labels/{label_id}",
            patch(labels::endpoints::update_label),
        )
        .route(
            "/{audit_id}/labels/{label_id}",
            delete(labels::endpoints::delete_label),
        )
        .route(
            "/{audit_id}/tickets/{ticket_id}/labels/{label_id}",
            post(labels::endpoints::attach_label),
        )
        .route(
            "/{audit_id}/tickets/{ticket_id}/labels/{label_id}",
            delete(labels::endpoints::detach_label),
        )
        // Comments
        .route(
            "/{audit_id}/tickets/{ticket_id}/comments",
            get(comments::endpoints::list_comments),
        )
        .route(
            "/{audit_id}/tickets/{ticket_id}/comments",
            post(comments::endpoints::create_comment),
        )
        .route(
            "/{audit_id}/tickets/{ticket_id}/comments/{comment_id}",
            patch(comments::endpoints::update_comment),
        )
        .route(
            "/{audit_id}/tickets/{ticket_id}/comments/{comment_id}",
            delete(comments::endpoints::delete_comment),
        )
        // Attachments
        .route(
            "/{audit_id}/tickets/{ticket_id}/attachments",
            get(super::attachments::endpoints::list_attachments),
        )
        .route(
            "/{audit_id}/tickets/{ticket_id}/attachments",
            post(super::attachments::endpoints::upload_attachment),
        )
        .route(
            "/{audit_id}/tickets/{ticket_id}/attachments/{attachment_id}",
            get(super::attachments::endpoints::download_attachment),
        )
        .route(
            "/{audit_id}/tickets/{ticket_id}/attachments/{attachment_id}",
            delete(super::attachments::endpoints::delete_attachment),
        )
        .layer(from_fn_with_state(state.clone(), audit_step_up_middleware))
        .layer(from_fn(auth::middleware))
}

/// Get the full board view for an audit
///
/// # Errors
/// Returns `TicketError` if database error
#[allow(clippy::too_many_lines)]
pub fn get_board(state: &GlobalState, audit_id: &str) -> Result<BoardResponse, TicketError> {
    use crate::schema::{
        kanban_columns, ticket_comments, ticket_labels, tickets, tickets_labels, users,
    };

    let mut conn = state.get_db_conn()?;

    // Load columns
    let db_columns = kanban_columns::table
        .filter(kanban_columns::audit_id.eq(audit_id))
        .order(kanban_columns::position.asc())
        .select(columns::models::KanbanColumn::as_select())
        .load::<columns::models::KanbanColumn>(&mut conn)?;

    // Load all tickets for this audit (only top-level, parent_id is NULL)
    let db_tickets = tickets::table
        .filter(tickets::audit_id.eq(audit_id))
        .filter(tickets::parent_id.is_null())
        .order(tickets::position.asc())
        .select(Ticket::as_select())
        .load::<Ticket>(&mut conn)?;

    let ticket_ids: Vec<String> = db_tickets.iter().map(|t| t.id.clone()).collect();

    // Load assignees and creators
    let user_ids: Vec<String> = db_tickets
        .iter()
        .flat_map(|t| {
            let mut ids = vec![t.created_by.clone()];
            if let Some(ref a) = t.assignee_id {
                ids.push(a.clone());
            }
            ids
        })
        .collect();

    let users_map: HashMap<String, User> = if user_ids.is_empty() {
        HashMap::new()
    } else {
        users::table
            .filter(users::id.eq_any(&user_ids))
            .select(User::as_select())
            .load::<User>(&mut conn)?
            .into_iter()
            .map(|u| (u.id.clone(), u))
            .collect()
    };

    // Load labels for all tickets
    let label_assignments: Vec<(String, TicketLabel)> = if ticket_ids.is_empty() {
        Vec::new()
    } else {
        tickets_labels::table
            .filter(tickets_labels::ticket_id.eq_any(&ticket_ids))
            .inner_join(ticket_labels::table)
            .select((tickets_labels::ticket_id, TicketLabel::as_select()))
            .load::<(String, TicketLabel)>(&mut conn)?
    };

    let mut labels_map: HashMap<String, Vec<LabelResponse>> = HashMap::new();
    for (ticket_id, label) in label_assignments {
        labels_map
            .entry(ticket_id)
            .or_default()
            .push(LabelResponse::from(label));
    }

    // Load subtask counts (load parent_ids and count in Rust)
    let subtask_counts: HashMap<String, i64> = if ticket_ids.is_empty() {
        HashMap::new()
    } else {
        let subtask_parent_ids: Vec<Option<String>> = tickets::table
            .filter(tickets::parent_id.eq_any(&ticket_ids))
            .select(tickets::parent_id)
            .load::<Option<String>>(&mut conn)?;

        let mut counts: HashMap<String, i64> = HashMap::new();
        for parent_id in subtask_parent_ids.into_iter().flatten() {
            *counts.entry(parent_id).or_insert(0) += 1;
        }
        counts
    };

    // Load comment counts (load ticket_ids and count in Rust)
    let comment_counts: HashMap<String, i64> = if ticket_ids.is_empty() {
        HashMap::new()
    } else {
        let comment_ticket_ids: Vec<String> = ticket_comments::table
            .filter(ticket_comments::ticket_id.eq_any(&ticket_ids))
            .select(ticket_comments::ticket_id)
            .load::<String>(&mut conn)?;

        let mut counts: HashMap<String, i64> = HashMap::new();
        for tid in comment_ticket_ids {
            *counts.entry(tid).or_insert(0) += 1;
        }
        counts
    };

    // Load attachment counts (load ticket_ids and count in Rust)
    let attachment_counts: HashMap<String, i64> = if ticket_ids.is_empty() {
        HashMap::new()
    } else {
        use crate::schema::ticket_attachments;
        let attachment_ticket_ids: Vec<String> = ticket_attachments::table
            .filter(ticket_attachments::ticket_id.eq_any(&ticket_ids))
            .select(ticket_attachments::ticket_id)
            .load::<String>(&mut conn)?;

        let mut counts: HashMap<String, i64> = HashMap::new();
        for tid in attachment_ticket_ids {
            *counts.entry(tid).or_insert(0) += 1;
        }
        counts
    };

    // Build board columns with tickets
    let board_columns: Vec<BoardColumn> = db_columns
        .into_iter()
        .map(|col| {
            let col_tickets: Vec<TicketSummary> = db_tickets
                .iter()
                .filter(|t| t.column_id == col.id)
                .map(|t| TicketSummary {
                    id: t.id.clone(),
                    column_id: t.column_id.clone(),
                    parent_id: t.parent_id.clone(),
                    title: t.title.clone(),
                    priority: t.priority.clone(),
                    assignee: t
                        .assignee_id
                        .as_ref()
                        .and_then(|aid| users_map.get(aid))
                        .map(|u| TicketUser::from(u.clone())),
                    position: t.position,
                    due_date: t.due_date.map(|d| d.to_string()),
                    labels: labels_map.get(&t.id).cloned().unwrap_or_default(),
                    subtask_count: subtask_counts.get(&t.id).copied().unwrap_or(0),
                    comment_count: comment_counts.get(&t.id).copied().unwrap_or(0),
                    attachment_count: attachment_counts.get(&t.id).copied().unwrap_or(0),
                })
                .collect();

            BoardColumn {
                id: col.id,
                name: col.name,
                position: col.position,
                tickets: col_tickets,
            }
        })
        .collect();

    // Load all audit labels
    let all_labels = labels::services::list_labels(state, audit_id)?;

    Ok(BoardResponse {
        audit_id: audit_id.to_string(),
        columns: board_columns,
        labels: all_labels,
    })
}

/// List tickets for an audit with optional filters
///
/// # Errors
/// Returns `TicketError` if database error
pub fn list_tickets(
    state: &GlobalState,
    audit_id: &str,
    column_filter: Option<&str>,
    priority_filter: Option<&str>,
    assignee_filter: Option<&str>,
) -> Result<Vec<Ticket>, TicketError> {
    use crate::schema::tickets;

    let mut conn = state.get_db_conn()?;

    let mut query = tickets::table
        .filter(tickets::audit_id.eq(audit_id))
        .filter(tickets::parent_id.is_null())
        .into_boxed();

    if let Some(col_id) = column_filter {
        query = query.filter(tickets::column_id.eq(col_id));
    }

    if let Some(priority) = priority_filter {
        query = query.filter(tickets::priority.eq(priority));
    }

    if let Some(assignee) = assignee_filter {
        query = query.filter(tickets::assignee_id.eq(assignee));
    }

    let result = query
        .order(tickets::position.asc())
        .select(Ticket::as_select())
        .load::<Ticket>(&mut conn)?;

    Ok(result)
}

/// Create a ticket
///
/// # Errors
/// Returns `TicketError` if database error or validation fails
pub fn create_ticket(
    state: &GlobalState,
    audit_id: &str,
    user_id: &str,
    payload: &CreateTicketPayload,
) -> Result<Ticket, TicketError> {
    use crate::schema::{kanban_columns, tickets};

    let audit = require_audit_not_archived(state, audit_id)?;
    require_team_member(state, &audit.team_id, user_id)?;

    let mut conn = state.get_db_conn()?;

    // Verify column belongs to audit
    let _column = kanban_columns::table
        .filter(kanban_columns::id.eq(&payload.column_id))
        .filter(kanban_columns::audit_id.eq(audit_id))
        .select(kanban_columns::id)
        .first::<String>(&mut conn)
        .optional()?
        .ok_or(TicketError::ColumnNotFound)?;

    // Verify parent exists and belongs to same audit (if specified)
    if let Some(ref parent_id) = payload.parent_id {
        let _parent = tickets::table
            .filter(tickets::id.eq(parent_id))
            .filter(tickets::audit_id.eq(audit_id))
            .select(tickets::id)
            .first::<String>(&mut conn)
            .optional()?
            .ok_or(TicketError::ParentNotFound)?;
    }

    // Get next position in column
    let max_position: Option<i32> = tickets::table
        .filter(tickets::column_id.eq(&payload.column_id))
        .select(diesel::dsl::max(tickets::position))
        .first::<Option<i32>>(&mut conn)?;

    let position = max_position.map_or(0, |p| p + 1);

    let priority = payload
        .priority
        .clone()
        .unwrap_or(super::models::TicketPriority::Medium);

    let ticket = Ticket::new(
        audit_id.to_string(),
        payload.column_id.clone(),
        payload.title.clone(),
        payload.description.clone(),
        priority,
        payload.assignee_id.clone(),
        position,
        payload.due_date,
        payload.parent_id.clone(),
        user_id.to_string(),
    );

    diesel::insert_into(tickets::table)
        .values(&ticket)
        .execute(&mut conn)?;

    Ok(ticket)
}

/// Get ticket by ID with full details
///
/// # Errors
/// Returns `TicketError` if database error or not found
pub fn get_ticket(
    state: &GlobalState,
    audit_id: &str,
    ticket_id: &str,
) -> Result<TicketResponse, TicketError> {
    use crate::schema::{ticket_attachments, ticket_comments, tickets, users};

    let mut conn = state.get_db_conn()?;

    let ticket = tickets::table
        .filter(tickets::id.eq(ticket_id))
        .filter(tickets::audit_id.eq(audit_id))
        .select(Ticket::as_select())
        .first::<Ticket>(&mut conn)
        .optional()?
        .ok_or(TicketError::NotFound)?;

    // Load creator
    let created_by = users::table
        .filter(users::id.eq(&ticket.created_by))
        .select(User::as_select())
        .first::<User>(&mut conn)?;

    // Load assignee
    let assignee = if let Some(ref assignee_id) = ticket.assignee_id {
        users::table
            .filter(users::id.eq(assignee_id))
            .select(User::as_select())
            .first::<User>(&mut conn)
            .optional()?
    } else {
        None
    };

    // Load labels
    let ticket_labels = labels::services::get_ticket_labels(state, ticket_id)?;

    // Subtask count
    let subtask_count = tickets::table
        .filter(tickets::parent_id.eq(ticket_id))
        .count()
        .get_result::<i64>(&mut conn)?;

    // Comment count
    let comment_count = ticket_comments::table
        .filter(ticket_comments::ticket_id.eq(ticket_id))
        .count()
        .get_result::<i64>(&mut conn)?;

    // Attachment count
    let attachment_count = ticket_attachments::table
        .filter(ticket_attachments::ticket_id.eq(ticket_id))
        .count()
        .get_result::<i64>(&mut conn)?;

    Ok(TicketResponse::from_ticket(
        ticket,
        assignee,
        created_by,
        ticket_labels,
        subtask_count,
        comment_count,
        attachment_count,
    ))
}

/// Update a ticket
///
/// # Errors
/// Returns `TicketError` if database error or not found
pub fn update_ticket(
    state: &GlobalState,
    audit_id: &str,
    ticket_id: &str,
    user_id: &str,
    payload: &UpdateTicketPayload,
) -> Result<(), TicketError> {
    use crate::schema::tickets;

    let audit = require_audit_not_archived(state, audit_id)?;
    require_team_member(state, &audit.team_id, user_id)?;

    let mut conn = state.get_db_conn()?;

    // Verify ticket exists
    let _ticket = tickets::table
        .filter(tickets::id.eq(ticket_id))
        .filter(tickets::audit_id.eq(audit_id))
        .select(tickets::id)
        .first::<String>(&mut conn)
        .optional()?
        .ok_or(TicketError::NotFound)?;

    // Validate parent_id if specified
    if let Some(ref parent_id) = payload.parent_id {
        if parent_id == ticket_id {
            return Err(TicketError::SelfParent);
        }
        let _parent = tickets::table
            .filter(tickets::id.eq(parent_id))
            .filter(tickets::audit_id.eq(audit_id))
            .select(tickets::id)
            .first::<String>(&mut conn)
            .optional()?
            .ok_or(TicketError::ParentNotFound)?;

        // Check for circular parent reference
        let mut current = parent_id.clone();
        for _ in 0..10 {
            let next_parent: Option<Option<String>> = tickets::table
                .filter(tickets::id.eq(&current))
                .select(tickets::parent_id)
                .first::<Option<String>>(&mut conn)
                .optional()?;

            match next_parent {
                Some(Some(pid)) if pid == ticket_id => {
                    return Err(TicketError::CircularParent);
                }
                Some(Some(pid)) => current = pid,
                _ => break,
            }
        }
    }

    // Apply individual field updates
    if let Some(ref title) = payload.title {
        diesel::update(tickets::table.filter(tickets::id.eq(ticket_id)))
            .set(tickets::title.eq(title))
            .execute(&mut conn)?;
    }

    if let Some(ref description) = payload.description {
        diesel::update(tickets::table.filter(tickets::id.eq(ticket_id)))
            .set(tickets::description.eq(description))
            .execute(&mut conn)?;
    }

    if let Some(ref priority) = payload.priority {
        diesel::update(tickets::table.filter(tickets::id.eq(ticket_id)))
            .set(tickets::priority.eq(priority))
            .execute(&mut conn)?;
    }

    if payload.clear_assignee {
        diesel::update(tickets::table.filter(tickets::id.eq(ticket_id)))
            .set(tickets::assignee_id.eq(None::<String>))
            .execute(&mut conn)?;
    } else if let Some(ref assignee_id) = payload.assignee_id {
        diesel::update(tickets::table.filter(tickets::id.eq(ticket_id)))
            .set(tickets::assignee_id.eq(assignee_id))
            .execute(&mut conn)?;
    }

    if payload.clear_parent {
        diesel::update(tickets::table.filter(tickets::id.eq(ticket_id)))
            .set(tickets::parent_id.eq(None::<String>))
            .execute(&mut conn)?;
    } else if let Some(ref parent_id) = payload.parent_id {
        diesel::update(tickets::table.filter(tickets::id.eq(ticket_id)))
            .set(tickets::parent_id.eq(parent_id))
            .execute(&mut conn)?;
    }

    if payload.clear_due_date {
        diesel::update(tickets::table.filter(tickets::id.eq(ticket_id)))
            .set(tickets::due_date.eq(None::<chrono::NaiveDateTime>))
            .execute(&mut conn)?;
    } else if let Some(ref due_date) = payload.due_date {
        diesel::update(tickets::table.filter(tickets::id.eq(ticket_id)))
            .set(tickets::due_date.eq(due_date))
            .execute(&mut conn)?;
    }

    Ok(())
}

/// Delete a ticket
///
/// # Errors
/// Returns `TicketError` if database error or not found
pub fn delete_ticket(
    state: &GlobalState,
    audit_id: &str,
    ticket_id: &str,
    user_id: &str,
    is_admin: bool,
) -> Result<(), TicketError> {
    use crate::schema::tickets;

    let audit = require_audit_not_archived(state, audit_id)?;
    require_team_member(state, &audit.team_id, user_id)?;

    let mut conn = state.get_db_conn()?;

    let ticket = tickets::table
        .filter(tickets::id.eq(ticket_id))
        .filter(tickets::audit_id.eq(audit_id))
        .select(Ticket::as_select())
        .first::<Ticket>(&mut conn)
        .optional()?
        .ok_or(TicketError::NotFound)?;

    // Only the creator, team owner/moderator, or admin can delete
    if ticket.created_by != user_id {
        require_owner_or_moderator(state, &audit.team_id, user_id, is_admin)?;
    }

    diesel::delete(tickets::table.filter(tickets::id.eq(ticket_id))).execute(&mut conn)?;

    Ok(())
}

/// Move a ticket to a different column/position
///
/// # Errors
/// Returns `TicketError` if database error or not found
pub fn move_ticket(
    state: &GlobalState,
    audit_id: &str,
    ticket_id: &str,
    user_id: &str,
    payload: &MoveTicketPayload,
) -> Result<(), TicketError> {
    use crate::schema::{kanban_columns, tickets};

    let audit = require_audit_not_archived(state, audit_id)?;
    require_team_member(state, &audit.team_id, user_id)?;

    let mut conn = state.get_db_conn()?;

    // Verify ticket exists
    let _ticket = tickets::table
        .filter(tickets::id.eq(ticket_id))
        .filter(tickets::audit_id.eq(audit_id))
        .select(tickets::id)
        .first::<String>(&mut conn)
        .optional()?
        .ok_or(TicketError::NotFound)?;

    // Verify column exists
    let column_audit: Option<String> = kanban_columns::table
        .filter(kanban_columns::id.eq(&payload.column_id))
        .select(kanban_columns::audit_id)
        .first::<String>(&mut conn)
        .optional()?
        .ok_or(TicketError::ColumnNotFound)
        .map(Some)?;

    // Verify column belongs to this audit
    if column_audit.as_deref() != Some(audit_id) {
        return Err(TicketError::ColumnAuditMismatch);
    }

    diesel::update(tickets::table.filter(tickets::id.eq(ticket_id)))
        .set((
            tickets::column_id.eq(&payload.column_id),
            tickets::position.eq(payload.position),
        ))
        .execute(&mut conn)?;

    Ok(())
}

/// List subtasks of a ticket
///
/// # Errors
/// Returns `TicketError` if database error or not found
pub fn list_subtasks(
    state: &GlobalState,
    audit_id: &str,
    ticket_id: &str,
) -> Result<Vec<Ticket>, TicketError> {
    use crate::schema::tickets;

    let mut conn = state.get_db_conn()?;

    // Verify parent exists
    let _ticket = tickets::table
        .filter(tickets::id.eq(ticket_id))
        .filter(tickets::audit_id.eq(audit_id))
        .select(tickets::id)
        .first::<String>(&mut conn)
        .optional()?
        .ok_or(TicketError::NotFound)?;

    let subtasks = tickets::table
        .filter(tickets::parent_id.eq(ticket_id))
        .order(tickets::position.asc())
        .select(Ticket::as_select())
        .load::<Ticket>(&mut conn)?;

    Ok(subtasks)
}
