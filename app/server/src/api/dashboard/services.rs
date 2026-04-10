use axum::middleware::from_fn;
use axum::routing::get;
use axum::Router;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

use crate::api::auth::admin_middleware;
use crate::schema::{
    audit_agents, audit_message_attachments, audit_messages, audits, audits_invitations, companies,
    events, teams, teams_invitations, teams_members, ticket_attachments, ticket_comments, tickets,
    users,
};
use crate::state::GlobalState;

use super::endpoints::get_dashboard;
use super::models::{
    AgentMetrics, AuditMetrics, CompanyMetrics, DashboardMetrics, EventMetrics, InvitationMetrics,
    InvitationStats, MessageMetrics, TeamMetrics, TicketMetrics, TicketsByPriority, UserMetrics,
    UsersByRole,
};

pub fn get_routes(state: &GlobalState) -> Router {
    Router::new()
        .route("/", get(get_dashboard))
        .layer(from_fn(admin_middleware))
        .with_state(state.clone())
}

pub fn collect_metrics(state: &GlobalState) -> Result<DashboardMetrics, diesel::result::Error> {
    let mut conn = state
        .get_db_conn()
        .map_err(|_| diesel::result::Error::NotFound)?;

    let now = Utc::now().naive_utc();
    let seven_days_ago = now - chrono::Duration::days(7);
    let thirty_days_ago = now - chrono::Duration::days(30);

    let user_metrics = collect_user_metrics(&mut conn, seven_days_ago, thirty_days_ago)?;
    let team_metrics = collect_team_metrics(&mut conn)?;
    let audit_metrics = collect_audit_metrics(&mut conn, now)?;
    let ticket_metrics = collect_ticket_metrics(&mut conn, now)?;
    let message_metrics =
        collect_message_metrics(&mut conn, seven_days_ago, thirty_days_ago)?;
    let event_metrics = collect_event_metrics(&mut conn, seven_days_ago, thirty_days_ago)?;
    let agent_metrics = collect_agent_metrics(state, &mut conn)?;
    let invitation_metrics = collect_invitation_metrics(&mut conn)?;
    let company_metrics = CompanyMetrics {
        total: companies::table.count().get_result(&mut conn)?,
    };

    Ok(DashboardMetrics {
        users: user_metrics,
        teams: team_metrics,
        audits: audit_metrics,
        tickets: ticket_metrics,
        messages: message_metrics,
        events: event_metrics,
        agents: agent_metrics,
        invitations: invitation_metrics,
        companies: company_metrics,
    })
}

fn collect_user_metrics(
    conn: &mut PgConnection,
    seven_days_ago: NaiveDateTime,
    thirty_days_ago: NaiveDateTime,
) -> Result<UserMetrics, diesel::result::Error> {
    let total: i64 = users::table.count().get_result(conn)?;

    let connected: i64 = users::table
        .filter(users::is_connected.eq(true))
        .count()
        .get_result(conn)?;

    let admin: i64 = users::table
        .filter(users::role.eq("Admin"))
        .count()
        .get_result(conn)?;

    let pentester: i64 = users::table
        .filter(users::role.eq("Pentester"))
        .count()
        .get_result(conn)?;

    let client: i64 = users::table
        .filter(users::role.eq("Client"))
        .count()
        .get_result(conn)?;

    let new_last_7_days: i64 = users::table
        .filter(users::created_at.ge(seven_days_ago))
        .count()
        .get_result(conn)?;

    let new_last_30_days: i64 = users::table
        .filter(users::created_at.ge(thirty_days_ago))
        .count()
        .get_result(conn)?;

    Ok(UserMetrics {
        total,
        connected,
        by_role: UsersByRole {
            admin,
            pentester,
            client,
        },
        new_last_7_days,
        new_last_30_days,
    })
}

#[allow(clippy::cast_precision_loss)]
fn collect_team_metrics(conn: &mut PgConnection) -> Result<TeamMetrics, diesel::result::Error> {
    let total: i64 = teams::table.count().get_result(conn)?;

    let total_members: i64 = teams_members::table.count().get_result(conn)?;

    let avg_members_per_team = if total > 0 {
        total_members as f64 / total as f64
    } else {
        0.0
    };

    Ok(TeamMetrics {
        total,
        total_members,
        avg_members_per_team,
    })
}

fn collect_audit_metrics(
    conn: &mut PgConnection,
    now: NaiveDateTime,
) -> Result<AuditMetrics, diesel::result::Error> {
    let total: i64 = audits::table.count().get_result(conn)?;

    let active: i64 = audits::table
        .filter(audits::is_archived.eq(false))
        .count()
        .get_result(conn)?;

    let archived: i64 = audits::table
        .filter(audits::is_archived.eq(true))
        .count()
        .get_result(conn)?;

    let seven_days_later = now + chrono::Duration::days(7);
    let expiring_in_7_days: i64 = audits::table
        .filter(audits::is_archived.eq(false))
        .filter(audits::end_date.is_not_null())
        .filter(audits::end_date.le(seven_days_later))
        .filter(audits::end_date.ge(now))
        .count()
        .get_result(conn)?;

    let with_end_date: i64 = audits::table
        .filter(audits::end_date.is_not_null())
        .count()
        .get_result(conn)?;

    let without_end_date: i64 = audits::table
        .filter(audits::end_date.is_null())
        .count()
        .get_result(conn)?;

    Ok(AuditMetrics {
        total,
        active,
        archived,
        expiring_in_7_days,
        with_end_date,
        without_end_date,
    })
}

fn collect_ticket_metrics(
    conn: &mut PgConnection,
    now: NaiveDateTime,
) -> Result<TicketMetrics, diesel::result::Error> {
    let total: i64 = tickets::table.count().get_result(conn)?;

    let critical: i64 = tickets::table
        .filter(tickets::priority.eq("CRITICAL"))
        .count()
        .get_result(conn)?;

    let high: i64 = tickets::table
        .filter(tickets::priority.eq("HIGH"))
        .count()
        .get_result(conn)?;

    let medium: i64 = tickets::table
        .filter(tickets::priority.eq("MEDIUM"))
        .count()
        .get_result(conn)?;

    let low: i64 = tickets::table
        .filter(tickets::priority.eq("LOW"))
        .count()
        .get_result(conn)?;

    let assigned: i64 = tickets::table
        .filter(tickets::assignee_id.is_not_null())
        .count()
        .get_result(conn)?;

    let unassigned: i64 = tickets::table
        .filter(tickets::assignee_id.is_null())
        .count()
        .get_result(conn)?;

    let overdue: i64 = tickets::table
        .filter(tickets::due_date.is_not_null())
        .filter(tickets::due_date.lt(now))
        .count()
        .get_result(conn)?;

    let total_comments: i64 = ticket_comments::table.count().get_result(conn)?;

    let total_attachments: i64 = ticket_attachments::table.count().get_result(conn)?;

    Ok(TicketMetrics {
        total,
        by_priority: TicketsByPriority {
            critical,
            high,
            medium,
            low,
        },
        assigned,
        unassigned,
        overdue,
        total_comments,
        total_attachments,
    })
}

fn collect_message_metrics(
    conn: &mut PgConnection,
    seven_days_ago: NaiveDateTime,
    thirty_days_ago: NaiveDateTime,
) -> Result<MessageMetrics, diesel::result::Error> {
    let total: i64 = audit_messages::table.count().get_result(conn)?;

    let last_7_days: i64 = audit_messages::table
        .filter(audit_messages::created_at.ge(seven_days_ago))
        .count()
        .get_result(conn)?;

    let last_30_days: i64 = audit_messages::table
        .filter(audit_messages::created_at.ge(thirty_days_ago))
        .count()
        .get_result(conn)?;

    let total_attachments: i64 = audit_message_attachments::table.count().get_result(conn)?;

    Ok(MessageMetrics {
        total,
        last_7_days,
        last_30_days,
        total_attachments,
    })
}

fn collect_event_metrics(
    conn: &mut PgConnection,
    seven_days_ago: NaiveDateTime,
    thirty_days_ago: NaiveDateTime,
) -> Result<EventMetrics, diesel::result::Error> {
    let total: i64 = events::table.count().get_result(conn)?;

    let last_7_days: i64 = events::table
        .filter(events::created_at.ge(seven_days_ago))
        .count()
        .get_result(conn)?;

    let last_30_days: i64 = events::table
        .filter(events::created_at.ge(thirty_days_ago))
        .count()
        .get_result(conn)?;

    Ok(EventMetrics {
        total,
        last_7_days,
        last_30_days,
    })
}

#[allow(clippy::cast_possible_wrap)]
fn collect_agent_metrics(
    state: &GlobalState,
    conn: &mut PgConnection,
) -> Result<AgentMetrics, diesel::result::Error> {
    let total: i64 = audit_agents::table.count().get_result(conn)?;

    let revoked: i64 = audit_agents::table
        .filter(audit_agents::is_revoked.eq(true))
        .count()
        .get_result(conn)?;

    let connected_count = state
        .agent_registry
        .lock()
        .map(|registry| registry.len() as i64)
        .unwrap_or(0);

    Ok(AgentMetrics {
        total,
        revoked,
        active: connected_count,
    })
}

fn collect_invitation_metrics(
    conn: &mut PgConnection,
) -> Result<InvitationMetrics, diesel::result::Error> {
    let teams_total: i64 = teams_invitations::table.count().get_result(conn)?;
    let teams_canceled: i64 = teams_invitations::table
        .filter(teams_invitations::is_canceled.eq(true))
        .count()
        .get_result(conn)?;
    let teams_fully_used: i64 = teams_invitations::table
        .filter(teams_invitations::is_canceled.eq(false))
        .filter(teams_invitations::used_count.ge(teams_invitations::max_uses))
        .count()
        .get_result(conn)?;
    let teams_active: i64 = teams_total - teams_canceled - teams_fully_used;

    let audits_total: i64 = audits_invitations::table.count().get_result(conn)?;
    let audits_canceled: i64 = audits_invitations::table
        .filter(audits_invitations::is_canceled.eq(true))
        .count()
        .get_result(conn)?;
    let audits_used: i64 = audits_invitations::table
        .filter(audits_invitations::is_used.eq(true))
        .count()
        .get_result(conn)?;
    let audits_active: i64 = audits_total - audits_canceled - audits_used;

    Ok(InvitationMetrics {
        teams: InvitationStats {
            total: teams_total,
            active: teams_active,
            canceled: teams_canceled,
            used: teams_fully_used,
        },
        audits: InvitationStats {
            total: audits_total,
            active: audits_active,
            canceled: audits_canceled,
            used: audits_used,
        },
    })
}
