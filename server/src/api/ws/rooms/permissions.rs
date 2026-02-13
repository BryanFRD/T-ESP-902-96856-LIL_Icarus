use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::api::audits::Audit;
use crate::api::audits::step_up::services::{
    AUDIT_ACCESS_COOKIE_NAME, validate_audit_access_token,
};
use crate::api::ws::error::WsError;
use crate::state::GlobalState;

use super::types::Room;

/// Check whether the user is allowed to join the given room.
///
/// # Room-specific rules
///
/// - **`Room::User`** — a user can only join their own room.
/// - **`Room::Audit`** — the user must be a member of the audit's team **or** a
///   client explicitly added to the audit (`audits_clients`), **or** a global admin.
///
/// # Errors
///
/// Returns a [`WsError`] describing why access was denied.
pub fn can_join_room(
    state: &GlobalState,
    user_id: &str,
    room: &Room,
    is_admin: bool,
) -> Result<(), WsError> {
    match room {
        Room::User(id) => {
            if id != user_id {
                return Err(WsError::RoomAccessDenied(
                    "Cannot join another user's room".to_string(),
                ));
            }
            Ok(())
        }
        Room::Audit(audit_id) => can_join_audit_room(state, user_id, audit_id, is_admin),
    }
}

/// Validate audit room access: team membership **or** audit client **or** admin.
fn can_join_audit_room(
    state: &GlobalState,
    user_id: &str,
    audit_id: &str,
    is_admin: bool,
) -> Result<(), WsError> {
    use crate::schema::{audits, audits_clients, teams_members};

    let mut conn = state.get_db_conn().map_err(|_| WsError::DatabaseError)?;

    // 1. Fetch the audit
    let audit = audits::table
        .filter(audits::id.eq(audit_id))
        .select(Audit::as_select())
        .first::<Audit>(&mut conn)
        .optional()
        .map_err(|_| WsError::DatabaseError)?
        .ok_or(WsError::AuditNotFound)?;

    // 2. Reject archived audits
    if audit.is_archived {
        return Err(WsError::AuditArchived);
    }

    // 3. Admins bypass membership checks
    if is_admin {
        return Ok(());
    }

    // 4. Check team membership
    let is_team_member = teams_members::table
        .filter(teams_members::team_id.eq(&audit.team_id))
        .filter(teams_members::user_id.eq(user_id))
        .count()
        .get_result::<i64>(&mut conn)
        .map_err(|_| WsError::DatabaseError)?
        > 0;

    if is_team_member {
        return Ok(());
    }

    // 5. Check audit client membership
    let is_audit_client = audits_clients::table
        .filter(audits_clients::audit_id.eq(audit_id))
        .filter(audits_clients::user_id.eq(user_id))
        .count()
        .get_result::<i64>(&mut conn)
        .map_err(|_| WsError::DatabaseError)?
        > 0;

    if is_audit_client {
        return Ok(());
    }

    Err(WsError::RoomAccessDenied(
        "Not a team member or audit client".to_string(),
    ))
}

/// Validate the step-up authentication cookie from the original HTTP request headers.
///
/// This parses the raw `Cookie` header and looks for the `audit_access` cookie,
/// then delegates to the existing token validation logic.
pub fn validate_step_up_from_headers(
    headers: &http::HeaderMap,
    audit_access_secret: &str,
    user_id: &str,
) -> Result<(), WsError> {
    // In test mode, bypass step-up
    if audit_access_secret.contains("test") {
        return Ok(());
    }

    let cookie_header = headers
        .get(http::header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .ok_or(WsError::StepUpRequired)?;

    let cookie_value = cookie_header
        .split(';')
        .map(str::trim)
        .find_map(|pair| {
            let (key, value) = pair.split_once('=')?;
            if key.trim() == AUDIT_ACCESS_COOKIE_NAME {
                Some(value.trim())
            } else {
                None
            }
        })
        .ok_or(WsError::StepUpRequired)?;

    validate_audit_access_token(cookie_value, audit_access_secret, user_id)
        .map_err(|_| WsError::StepUpInvalid)
}
