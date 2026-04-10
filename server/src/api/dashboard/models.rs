use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DashboardMetrics {
    pub users: UserMetrics,
    pub teams: TeamMetrics,
    pub audits: AuditMetrics,
    pub tickets: TicketMetrics,
    pub messages: MessageMetrics,
    pub events: EventMetrics,
    pub agents: AgentMetrics,
    pub invitations: InvitationMetrics,
    pub companies: CompanyMetrics,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserMetrics {
    pub total: i64,
    pub connected: i64,
    pub by_role: UsersByRole,
    pub new_last_7_days: i64,
    pub new_last_30_days: i64,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UsersByRole {
    pub admin: i64,
    pub pentester: i64,
    pub client: i64,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TeamMetrics {
    pub total: i64,
    pub total_members: i64,
    pub avg_members_per_team: f64,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuditMetrics {
    pub total: i64,
    pub active: i64,
    pub archived: i64,
    pub expiring_in_7_days: i64,
    pub with_end_date: i64,
    pub without_end_date: i64,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TicketMetrics {
    pub total: i64,
    pub by_priority: TicketsByPriority,
    pub assigned: i64,
    pub unassigned: i64,
    pub overdue: i64,
    pub total_comments: i64,
    pub total_attachments: i64,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TicketsByPriority {
    pub critical: i64,
    pub high: i64,
    pub medium: i64,
    pub low: i64,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MessageMetrics {
    pub total: i64,
    pub last_7_days: i64,
    pub last_30_days: i64,
    pub total_attachments: i64,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EventMetrics {
    pub total: i64,
    pub last_7_days: i64,
    pub last_30_days: i64,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AgentMetrics {
    pub total: i64,
    pub revoked: i64,
    pub active: i64,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct InvitationMetrics {
    pub teams: InvitationStats,
    pub audits: InvitationStats,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct InvitationStats {
    pub total: i64,
    pub active: i64,
    pub canceled: i64,
    pub used: i64,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CompanyMetrics {
    pub total: i64,
}
