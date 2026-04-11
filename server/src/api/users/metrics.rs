use axum::{Json, extract::State, response::IntoResponse};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use http::StatusCode;
use serde::Serialize;
use utoipa::ToSchema;

use crate::api::auth::JwtClaims;
use crate::schema::{
    audit_messages, audits, audits_clients, events, teams_members, ticket_comments, tickets,
};
use crate::state::GlobalState;

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserDashboardMetrics {
    pub teams: UserTeamMetrics,
    pub audits: UserAuditMetrics,
    pub tickets: UserTicketMetrics,
    pub messages: UserMessageMetrics,
    pub comments: UserCommentMetrics,
    pub events: UserEventMetrics,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserTeamMetrics {
    pub total: i64,
    pub as_owner: i64,
    pub as_member: i64,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserAuditMetrics {
    pub as_team_member: i64,
    pub as_client: i64,
    pub active: i64,
    pub archived: i64,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserTicketMetrics {
    pub assigned: i64,
    pub created: i64,
    pub overdue: i64,
    pub by_priority: UserTicketsByPriority,
    pub by_priority_assigned: UserTicketsByPriority,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserTicketsByPriority {
    pub critical: i64,
    pub high: i64,
    pub medium: i64,
    pub low: i64,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserMessageMetrics {
    pub total: i64,
    pub last_7_days: i64,
    pub last_30_days: i64,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserCommentMetrics {
    pub total: i64,
    pub last_7_days: i64,
    pub last_30_days: i64,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserEventMetrics {
    pub total: i64,
    pub last_7_days: i64,
    pub last_30_days: i64,
}

#[utoipa::path(
    get,
    path = "/v1/users/me/metrics",
    description = "Get metrics for the authenticated user",
    responses(
        (status = 200, description = "User metrics retrieved successfully", body = UserDashboardMetrics),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "User",
    security(("cookieAuth" = []))
)]
pub async fn me_metrics(State(state): State<GlobalState>, jwt: JwtClaims) -> impl IntoResponse {
    let user_id = jwt.sub.to_string();

    match collect_user_metrics(&state, &user_id) {
        Ok(metrics) => (StatusCode::OK, Json(metrics)).into_response(),
        Err(e) => {
            tracing::error!("Failed to collect user metrics: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to collect user metrics"})),
            )
                .into_response()
        }
    }
}

fn collect_user_metrics(
    state: &GlobalState,
    user_id: &str,
) -> Result<UserDashboardMetrics, diesel::result::Error> {
    let mut conn = state
        .get_db_conn()
        .map_err(|_| diesel::result::Error::NotFound)?;

    let now = Utc::now().naive_utc();
    let seven_days_ago = now - chrono::Duration::days(7);
    let thirty_days_ago = now - chrono::Duration::days(30);

    let teams = collect_team_metrics(&mut conn, user_id)?;
    let audits = collect_audit_metrics(&mut conn, user_id)?;
    let tickets = collect_ticket_metrics(&mut conn, user_id, now)?;
    let messages = collect_message_metrics(&mut conn, user_id, seven_days_ago, thirty_days_ago)?;
    let comments = collect_comment_metrics(&mut conn, user_id, seven_days_ago, thirty_days_ago)?;
    let events = collect_event_metrics(&mut conn, user_id, seven_days_ago, thirty_days_ago)?;

    Ok(UserDashboardMetrics {
        teams,
        audits,
        tickets,
        messages,
        comments,
        events,
    })
}

fn collect_team_metrics(
    conn: &mut PgConnection,
    user_id: &str,
) -> Result<UserTeamMetrics, diesel::result::Error> {
    let total: i64 = teams_members::table
        .filter(teams_members::user_id.eq(user_id))
        .count()
        .get_result(conn)?;

    let as_owner: i64 = teams_members::table
        .filter(teams_members::user_id.eq(user_id))
        .filter(teams_members::role.eq("owner"))
        .count()
        .get_result(conn)?;

    let as_member = total - as_owner;

    Ok(UserTeamMetrics {
        total,
        as_owner,
        as_member,
    })
}

fn collect_audit_metrics(
    conn: &mut PgConnection,
    user_id: &str,
) -> Result<UserAuditMetrics, diesel::result::Error> {
    // Audits where user is part of the team
    let user_team_ids: Vec<String> = teams_members::table
        .filter(teams_members::user_id.eq(user_id))
        .select(teams_members::team_id)
        .load(conn)?;

    let as_team_member: i64 = audits::table
        .filter(audits::team_id.eq_any(&user_team_ids))
        .count()
        .get_result(conn)?;

    let as_client: i64 = audits_clients::table
        .filter(audits_clients::user_id.eq(user_id))
        .count()
        .get_result(conn)?;

    let active: i64 = audits::table
        .filter(audits::team_id.eq_any(&user_team_ids))
        .filter(audits::is_archived.eq(false))
        .count()
        .get_result(conn)?;

    let archived: i64 = audits::table
        .filter(audits::team_id.eq_any(&user_team_ids))
        .filter(audits::is_archived.eq(true))
        .count()
        .get_result(conn)?;

    Ok(UserAuditMetrics {
        as_team_member,
        as_client,
        active,
        archived,
    })
}

fn collect_ticket_metrics(
    conn: &mut PgConnection,
    user_id: &str,
    now: NaiveDateTime,
) -> Result<UserTicketMetrics, diesel::result::Error> {
    // Get audit IDs the user has access to (via team membership)
    let user_team_ids: Vec<String> = teams_members::table
        .filter(teams_members::user_id.eq(user_id))
        .select(teams_members::team_id)
        .load(conn)?;

    let user_audit_ids: Vec<String> = audits::table
        .filter(audits::team_id.eq_any(&user_team_ids))
        .select(audits::id)
        .load(conn)?;

    let assigned: i64 = tickets::table
        .filter(tickets::assignee_id.eq(user_id))
        .count()
        .get_result(conn)?;

    let created: i64 = tickets::table
        .filter(tickets::created_by.eq(user_id))
        .count()
        .get_result(conn)?;

    let overdue: i64 = tickets::table
        .filter(tickets::audit_id.eq_any(&user_audit_ids))
        .filter(tickets::due_date.is_not_null())
        .filter(tickets::due_date.lt(now))
        .count()
        .get_result(conn)?;

    // Count by priority across all user's audits (total)
    // Priority values are stored lowercase in DB
    let critical: i64 = tickets::table
        .filter(tickets::audit_id.eq_any(&user_audit_ids))
        .filter(tickets::priority.eq("critical"))
        .count()
        .get_result(conn)?;

    let high: i64 = tickets::table
        .filter(tickets::audit_id.eq_any(&user_audit_ids))
        .filter(tickets::priority.eq("high"))
        .count()
        .get_result(conn)?;

    let medium: i64 = tickets::table
        .filter(tickets::audit_id.eq_any(&user_audit_ids))
        .filter(tickets::priority.eq("medium"))
        .count()
        .get_result(conn)?;

    let low: i64 = tickets::table
        .filter(tickets::audit_id.eq_any(&user_audit_ids))
        .filter(tickets::priority.eq("low"))
        .count()
        .get_result(conn)?;

    // Count by priority assigned to user
    let critical_assigned: i64 = tickets::table
        .filter(tickets::assignee_id.eq(user_id))
        .filter(tickets::priority.eq("critical"))
        .count()
        .get_result(conn)?;

    let high_assigned: i64 = tickets::table
        .filter(tickets::assignee_id.eq(user_id))
        .filter(tickets::priority.eq("high"))
        .count()
        .get_result(conn)?;

    let medium_assigned: i64 = tickets::table
        .filter(tickets::assignee_id.eq(user_id))
        .filter(tickets::priority.eq("medium"))
        .count()
        .get_result(conn)?;

    let low_assigned: i64 = tickets::table
        .filter(tickets::assignee_id.eq(user_id))
        .filter(tickets::priority.eq("low"))
        .count()
        .get_result(conn)?;

    Ok(UserTicketMetrics {
        assigned,
        created,
        overdue,
        by_priority: UserTicketsByPriority {
            critical,
            high,
            medium,
            low,
        },
        by_priority_assigned: UserTicketsByPriority {
            critical: critical_assigned,
            high: high_assigned,
            medium: medium_assigned,
            low: low_assigned,
        },
    })
}

fn collect_message_metrics(
    conn: &mut PgConnection,
    user_id: &str,
    seven_days_ago: NaiveDateTime,
    thirty_days_ago: NaiveDateTime,
) -> Result<UserMessageMetrics, diesel::result::Error> {
    let total: i64 = audit_messages::table
        .filter(audit_messages::author_id.eq(user_id))
        .count()
        .get_result(conn)?;

    let last_7_days: i64 = audit_messages::table
        .filter(audit_messages::author_id.eq(user_id))
        .filter(audit_messages::created_at.ge(seven_days_ago))
        .count()
        .get_result(conn)?;

    let last_30_days: i64 = audit_messages::table
        .filter(audit_messages::author_id.eq(user_id))
        .filter(audit_messages::created_at.ge(thirty_days_ago))
        .count()
        .get_result(conn)?;

    Ok(UserMessageMetrics {
        total,
        last_7_days,
        last_30_days,
    })
}

fn collect_comment_metrics(
    conn: &mut PgConnection,
    user_id: &str,
    seven_days_ago: NaiveDateTime,
    thirty_days_ago: NaiveDateTime,
) -> Result<UserCommentMetrics, diesel::result::Error> {
    let total: i64 = ticket_comments::table
        .filter(ticket_comments::author_id.eq(user_id))
        .count()
        .get_result(conn)?;

    let last_7_days: i64 = ticket_comments::table
        .filter(ticket_comments::author_id.eq(user_id))
        .filter(ticket_comments::created_at.ge(seven_days_ago))
        .count()
        .get_result(conn)?;

    let last_30_days: i64 = ticket_comments::table
        .filter(ticket_comments::author_id.eq(user_id))
        .filter(ticket_comments::created_at.ge(thirty_days_ago))
        .count()
        .get_result(conn)?;

    Ok(UserCommentMetrics {
        total,
        last_7_days,
        last_30_days,
    })
}

fn collect_event_metrics(
    conn: &mut PgConnection,
    user_id: &str,
    seven_days_ago: NaiveDateTime,
    thirty_days_ago: NaiveDateTime,
) -> Result<UserEventMetrics, diesel::result::Error> {
    let total: i64 = events::table
        .filter(events::user_id.eq(user_id))
        .count()
        .get_result(conn)?;

    let last_7_days: i64 = events::table
        .filter(events::user_id.eq(user_id))
        .filter(events::created_at.ge(seven_days_ago))
        .count()
        .get_result(conn)?;

    let last_30_days: i64 = events::table
        .filter(events::user_id.eq(user_id))
        .filter(events::created_at.ge(thirty_days_ago))
        .count()
        .get_result(conn)?;

    Ok(UserEventMetrics {
        total,
        last_7_days,
        last_30_days,
    })
}
