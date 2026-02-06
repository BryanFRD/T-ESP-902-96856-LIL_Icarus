use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::state::GlobalState;

use super::models::{
    ColumnResponse, CreateColumnPayload, KanbanColumn, ReorderColumnsPayload, UpdateColumnPayload,
};
use crate::api::audits::tickets::error::TicketError;

/// Seed default kanban columns for a new audit
///
/// # Errors
/// Returns `TicketError` if database error
pub fn seed_default_columns(
    conn: &mut diesel::PgConnection,
    audit_id: &str,
) -> Result<(), TicketError> {
    use crate::schema::kanban_columns;

    let defaults = [("To Do", 0), ("In Progress", 1), ("Done", 2)];

    for (name, position) in &defaults {
        let column = KanbanColumn::new(audit_id.to_string(), (*name).to_string(), *position);
        diesel::insert_into(kanban_columns::table)
            .values(&column)
            .execute(conn)?;
    }

    Ok(())
}

/// List all columns for an audit
///
/// # Errors
/// Returns `TicketError` if database error
pub fn list_columns(
    state: &GlobalState,
    audit_id: &str,
) -> Result<Vec<ColumnResponse>, TicketError> {
    use crate::schema::kanban_columns;

    let mut conn = state.get_db_conn()?;
    let columns = kanban_columns::table
        .filter(kanban_columns::audit_id.eq(audit_id))
        .order(kanban_columns::position.asc())
        .select(KanbanColumn::as_select())
        .load::<KanbanColumn>(&mut conn)?;

    Ok(columns.into_iter().map(ColumnResponse::from).collect())
}

/// Create a new column in an audit
///
/// # Errors
/// Returns `TicketError` if database error or duplicate name
pub fn create_column(
    state: &GlobalState,
    audit_id: &str,
    payload: &CreateColumnPayload,
) -> Result<ColumnResponse, TicketError> {
    use crate::schema::kanban_columns;

    let mut conn = state.get_db_conn()?;

    // Check for duplicate name
    let exists = kanban_columns::table
        .filter(kanban_columns::audit_id.eq(audit_id))
        .filter(kanban_columns::name.eq(&payload.name))
        .count()
        .get_result::<i64>(&mut conn)?
        > 0;

    if exists {
        return Err(TicketError::DuplicateColumnName);
    }

    // Get next position
    let max_position: Option<i32> = kanban_columns::table
        .filter(kanban_columns::audit_id.eq(audit_id))
        .select(diesel::dsl::max(kanban_columns::position))
        .first::<Option<i32>>(&mut conn)?;

    let position = max_position.map_or(0, |p| p + 1);

    let column = KanbanColumn::new(audit_id.to_string(), payload.name.clone(), position);
    diesel::insert_into(kanban_columns::table)
        .values(&column)
        .execute(&mut conn)?;

    Ok(ColumnResponse::from(column))
}

/// Update a column name
///
/// # Errors
/// Returns `TicketError` if database error, not found, or duplicate name
pub fn update_column(
    state: &GlobalState,
    audit_id: &str,
    column_id: &str,
    payload: &UpdateColumnPayload,
) -> Result<ColumnResponse, TicketError> {
    use crate::schema::kanban_columns;

    let mut conn = state.get_db_conn()?;

    let column = kanban_columns::table
        .filter(kanban_columns::id.eq(column_id))
        .filter(kanban_columns::audit_id.eq(audit_id))
        .select(KanbanColumn::as_select())
        .first::<KanbanColumn>(&mut conn)
        .optional()?
        .ok_or(TicketError::ColumnNotFound)?;

    // Check for duplicate name (excluding current column)
    let exists = kanban_columns::table
        .filter(kanban_columns::audit_id.eq(audit_id))
        .filter(kanban_columns::name.eq(&payload.name))
        .filter(kanban_columns::id.ne(column_id))
        .count()
        .get_result::<i64>(&mut conn)?
        > 0;

    if exists {
        return Err(TicketError::DuplicateColumnName);
    }

    diesel::update(kanban_columns::table.filter(kanban_columns::id.eq(column_id)))
        .set(kanban_columns::name.eq(&payload.name))
        .execute(&mut conn)?;

    let updated = KanbanColumn {
        name: payload.name.clone(),
        ..column
    };

    Ok(ColumnResponse::from(updated))
}

/// Delete a column (must be empty)
///
/// # Errors
/// Returns `TicketError` if database error, not found, or column has tickets
pub fn delete_column(
    state: &GlobalState,
    audit_id: &str,
    column_id: &str,
) -> Result<(), TicketError> {
    use crate::schema::{kanban_columns, tickets};

    let mut conn = state.get_db_conn()?;

    // Verify column exists and belongs to audit
    let _column = kanban_columns::table
        .filter(kanban_columns::id.eq(column_id))
        .filter(kanban_columns::audit_id.eq(audit_id))
        .select(KanbanColumn::as_select())
        .first::<KanbanColumn>(&mut conn)
        .optional()?
        .ok_or(TicketError::ColumnNotFound)?;

    // Check if column has tickets
    let ticket_count = tickets::table
        .filter(tickets::column_id.eq(column_id))
        .count()
        .get_result::<i64>(&mut conn)?;

    if ticket_count > 0 {
        return Err(TicketError::ColumnHasTickets);
    }

    diesel::delete(kanban_columns::table.filter(kanban_columns::id.eq(column_id)))
        .execute(&mut conn)?;

    Ok(())
}

/// Reorder columns
///
/// # Errors
/// Returns `TicketError` if database error
pub fn reorder_columns(
    state: &GlobalState,
    audit_id: &str,
    payload: &ReorderColumnsPayload,
) -> Result<Vec<ColumnResponse>, TicketError> {
    use crate::schema::kanban_columns;

    let mut conn = state.get_db_conn()?;

    for (index, column_id) in payload.column_ids.iter().enumerate() {
        let position = i32::try_from(index).unwrap_or(0);
        diesel::update(
            kanban_columns::table
                .filter(kanban_columns::id.eq(column_id))
                .filter(kanban_columns::audit_id.eq(audit_id)),
        )
        .set(kanban_columns::position.eq(position))
        .execute(&mut conn)?;
    }

    list_columns(state, audit_id)
}
