use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::state::GlobalState;

use super::models::{
    CreateLabelPayload, LabelResponse, TicketLabel, TicketLabelAssignment, UpdateLabelPayload,
};
use crate::api::audits::tickets::error::TicketError;

/// List all labels for an audit
///
/// # Errors
/// Returns `TicketError` if database error
pub fn list_labels(state: &GlobalState, audit_id: &str) -> Result<Vec<LabelResponse>, TicketError> {
    use crate::schema::ticket_labels;

    let mut conn = state.get_db_conn()?;
    let labels = ticket_labels::table
        .filter(ticket_labels::audit_id.eq(audit_id))
        .order(ticket_labels::name.asc())
        .select(TicketLabel::as_select())
        .load::<TicketLabel>(&mut conn)?;

    Ok(labels.into_iter().map(LabelResponse::from).collect())
}

/// Create a new label
///
/// # Errors
/// Returns `TicketError` if database error or duplicate name
pub fn create_label(
    state: &GlobalState,
    audit_id: &str,
    payload: &CreateLabelPayload,
) -> Result<LabelResponse, TicketError> {
    use crate::schema::ticket_labels;

    let mut conn = state.get_db_conn()?;

    // Check for duplicate name
    let exists = ticket_labels::table
        .filter(ticket_labels::audit_id.eq(audit_id))
        .filter(ticket_labels::name.eq(&payload.name))
        .count()
        .get_result::<i64>(&mut conn)?
        > 0;

    if exists {
        return Err(TicketError::DuplicateLabelName);
    }

    let color = payload
        .color
        .clone()
        .unwrap_or_else(|| "#6B7280".to_string());

    let label = TicketLabel::new(audit_id.to_string(), payload.name.clone(), color);
    diesel::insert_into(ticket_labels::table)
        .values(&label)
        .execute(&mut conn)?;

    Ok(LabelResponse::from(label))
}

/// Update a label
///
/// # Errors
/// Returns `TicketError` if database error, not found, or duplicate name
pub fn update_label(
    state: &GlobalState,
    audit_id: &str,
    label_id: &str,
    payload: &UpdateLabelPayload,
) -> Result<LabelResponse, TicketError> {
    use crate::schema::ticket_labels;

    let mut conn = state.get_db_conn()?;

    let label = ticket_labels::table
        .filter(ticket_labels::id.eq(label_id))
        .filter(ticket_labels::audit_id.eq(audit_id))
        .select(TicketLabel::as_select())
        .first::<TicketLabel>(&mut conn)
        .optional()?
        .ok_or(TicketError::LabelNotFound)?;

    // Check for duplicate name (if name is being changed)
    if let Some(ref new_name) = payload.name {
        let exists = ticket_labels::table
            .filter(ticket_labels::audit_id.eq(audit_id))
            .filter(ticket_labels::name.eq(new_name))
            .filter(ticket_labels::id.ne(label_id))
            .count()
            .get_result::<i64>(&mut conn)?
            > 0;

        if exists {
            return Err(TicketError::DuplicateLabelName);
        }
    }

    let update_fields = (
        payload.name.as_ref().map(|n| ticket_labels::name.eq(n)),
        payload.color.as_ref().map(|c| ticket_labels::color.eq(c)),
    );

    diesel::update(ticket_labels::table.filter(ticket_labels::id.eq(label_id)))
        .set(update_fields)
        .execute(&mut conn)?;

    let updated = TicketLabel {
        name: payload.name.clone().unwrap_or(label.name),
        color: payload.color.clone().unwrap_or(label.color),
        ..label
    };

    Ok(LabelResponse::from(updated))
}

/// Delete a label
///
/// # Errors
/// Returns `TicketError` if database error or not found
pub fn delete_label(
    state: &GlobalState,
    audit_id: &str,
    label_id: &str,
) -> Result<(), TicketError> {
    use crate::schema::ticket_labels;

    let mut conn = state.get_db_conn()?;

    let _label = ticket_labels::table
        .filter(ticket_labels::id.eq(label_id))
        .filter(ticket_labels::audit_id.eq(audit_id))
        .select(TicketLabel::as_select())
        .first::<TicketLabel>(&mut conn)
        .optional()?
        .ok_or(TicketError::LabelNotFound)?;

    diesel::delete(ticket_labels::table.filter(ticket_labels::id.eq(label_id)))
        .execute(&mut conn)?;

    Ok(())
}

/// Attach a label to a ticket
///
/// # Errors
/// Returns `TicketError` if database error, not found, or already attached
pub fn attach_label(
    state: &GlobalState,
    audit_id: &str,
    ticket_id: &str,
    label_id: &str,
) -> Result<(), TicketError> {
    use crate::schema::{ticket_labels, tickets, tickets_labels};

    let mut conn = state.get_db_conn()?;

    // Verify ticket belongs to audit
    let _ticket = tickets::table
        .filter(tickets::id.eq(ticket_id))
        .filter(tickets::audit_id.eq(audit_id))
        .select(tickets::id)
        .first::<String>(&mut conn)
        .optional()?
        .ok_or(TicketError::NotFound)?;

    // Verify label exists
    let label_audit_id = ticket_labels::table
        .filter(ticket_labels::id.eq(label_id))
        .select(ticket_labels::audit_id)
        .first::<String>(&mut conn)
        .optional()?
        .ok_or(TicketError::LabelNotFound)?;

    // Verify label belongs to this audit
    if label_audit_id != audit_id {
        return Err(TicketError::LabelAuditMismatch);
    }

    // Check if already attached
    let already = tickets_labels::table
        .filter(tickets_labels::ticket_id.eq(ticket_id))
        .filter(tickets_labels::label_id.eq(label_id))
        .count()
        .get_result::<i64>(&mut conn)?
        > 0;

    if already {
        return Err(TicketError::LabelAlreadyAttached);
    }

    let assignment = TicketLabelAssignment::new(ticket_id.to_string(), label_id.to_string());
    diesel::insert_into(tickets_labels::table)
        .values(&assignment)
        .execute(&mut conn)?;

    Ok(())
}

/// Detach a label from a ticket
///
/// # Errors
/// Returns `TicketError` if database error or not attached
pub fn detach_label(
    state: &GlobalState,
    audit_id: &str,
    ticket_id: &str,
    label_id: &str,
) -> Result<(), TicketError> {
    use crate::schema::{tickets, tickets_labels};

    let mut conn = state.get_db_conn()?;

    // Verify ticket belongs to audit
    let _ticket = tickets::table
        .filter(tickets::id.eq(ticket_id))
        .filter(tickets::audit_id.eq(audit_id))
        .select(tickets::id)
        .first::<String>(&mut conn)
        .optional()?
        .ok_or(TicketError::NotFound)?;

    let deleted = diesel::delete(
        tickets_labels::table
            .filter(tickets_labels::ticket_id.eq(ticket_id))
            .filter(tickets_labels::label_id.eq(label_id)),
    )
    .execute(&mut conn)?;

    if deleted == 0 {
        return Err(TicketError::LabelNotAttached);
    }

    Ok(())
}

/// Get labels for a specific ticket
///
/// # Errors
/// Returns `TicketError` if database error
pub fn get_ticket_labels(
    state: &GlobalState,
    ticket_id: &str,
) -> Result<Vec<LabelResponse>, TicketError> {
    use crate::schema::{ticket_labels, tickets_labels};

    let mut conn = state.get_db_conn()?;
    let labels = tickets_labels::table
        .filter(tickets_labels::ticket_id.eq(ticket_id))
        .inner_join(ticket_labels::table)
        .select(TicketLabel::as_select())
        .load::<TicketLabel>(&mut conn)?;

    Ok(labels.into_iter().map(LabelResponse::from).collect())
}
