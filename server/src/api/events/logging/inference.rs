use crate::api::events::{EventAction, ResourceType};

use super::route_pattern::RoutePattern;

const TEAM_ID: &str = "team_id";
const USER_ID: &str = "user_id";
const COMPANY_ID: &str = "company_id";
const TOKEN: &str = "token";
const AUDIT_ID: &str = "audit_id";
const TICKET_ID: &str = "ticket_id";
const INVITATION_ID: &str = "invitation_id";
const COLUMN_ID: &str = "column_id";
const LABEL_ID: &str = "label_id";

macro_rules! event {
    ($action:ident, $resource:ident) => {
        Some((EventAction::$action, ResourceType::$resource, None))
    };
}

macro_rules! event_with_id {
    ($action:ident, $resource:ident, $($key:expr => $value:expr),+) => {
        Some((
            EventAction::$action,
            ResourceType::$resource,
            Some(vec![$(format!("{}:{}", $key, $value)),+].join(",")),
        ))
    };
}

pub fn infer_post_event(
    route: &RoutePattern,
) -> Option<(EventAction, ResourceType, Option<String>)> {
    match (
        route.get(0),
        route.get(1),
        route.get(2),
        route.get(3),
        route.get(4),
        route.get(5),
    ) {
        (Some("auth"), Some("login"), _, _, _, _) => event!(Login, Auth),
        (Some("auth"), Some("register"), _, _, _, _) => event!(Register, Auth),
        (Some("auth"), Some("logout"), _, _, _, _) => event!(Logout, Auth),
        (Some("auth"), Some("reset-password"), _, _, _, _) => event!(PasswordReset, Auth),
        (Some("users"), Some("me"), Some("avatar"), _, _, _) => event!(AvatarUpload, Avatar),
        (Some("teams"), Some("me"), _, _, _, _) => event!(Create, Team),
        (Some("teams"), Some(team_id), Some("company"), _, _, _) => {
            event_with_id!(Create, Company, TEAM_ID => team_id)
        }
        (Some("teams"), Some(team_id), Some("invitations"), _, _, _)
            if team_id != "invitations" =>
        {
            event_with_id!(InvitationCreate, Invitation, TEAM_ID => team_id)
        }
        (Some("teams"), Some("invitations"), Some("join"), _, _, _) => {
            event!(InvitationConsume, Invitation)
        }
        // Audit routes
        (Some("audits"), Some("me"), _, _, _, _) => event!(Create, Audit),
        (Some("audits"), Some(audit_id), Some("invite"), None, _, _)
            if audit_id != "me"
                && audit_id != "verify-access"
                && audit_id != "check-access"
                && audit_id != "invite" =>
        {
            event_with_id!(InvitationCreate, AuditInvitation, AUDIT_ID => audit_id)
        }
        (Some("audits"), Some("invite"), Some(token), Some("accept"), _, _) => {
            event_with_id!(InvitationConsume, AuditInvitation, TOKEN => token)
        }
        (
            Some("audits"),
            Some(audit_id),
            Some("tickets"),
            Some(ticket_id),
            Some("comments"),
            None,
        ) if audit_id != "me" => {
            event_with_id!(Create, TicketComment, AUDIT_ID => audit_id, TICKET_ID => ticket_id)
        }
        (
            Some("audits"),
            Some(audit_id),
            Some("tickets"),
            Some(ticket_id),
            Some("attachments"),
            None,
        ) if audit_id != "me" => {
            event_with_id!(Create, TicketAttachment, AUDIT_ID => audit_id, TICKET_ID => ticket_id)
        }
        (
            Some("audits"),
            Some(audit_id),
            Some("tickets"),
            Some(ticket_id),
            Some("labels"),
            Some(label_id),
        ) if audit_id != "me" => {
            event_with_id!(Create, TicketLabel, AUDIT_ID => audit_id, TICKET_ID => ticket_id, LABEL_ID => label_id)
        }
        (Some("audits"), Some(audit_id), Some("tickets"), None, _, _) if audit_id != "me" => {
            event_with_id!(Create, Ticket, AUDIT_ID => audit_id)
        }
        (Some("audits"), Some(audit_id), Some("columns"), None, _, _) if audit_id != "me" => {
            event_with_id!(Create, KanbanColumn, AUDIT_ID => audit_id)
        }
        (Some("audits"), Some(audit_id), Some("labels"), None, _, _) if audit_id != "me" => {
            event_with_id!(Create, TicketLabel, AUDIT_ID => audit_id)
        }
        // Agent routes
        (Some("audits"), Some(audit_id), Some("agents"), Some(agent_id), Some("shell"), _)
            if audit_id != "me" =>
        {
            event_with_id!(AgentCommand, Agent, AUDIT_ID => audit_id, "agent_id" => agent_id)
        }
        (Some("audits"), Some(audit_id), Some("agents"), Some(agent_id), Some("keylogger"), _)
            if audit_id != "me" =>
        {
            event_with_id!(AgentCommand, Agent, AUDIT_ID => audit_id, "agent_id" => agent_id)
        }
        // Audit messages
        (Some("audits"), Some(audit_id), Some("messages"), None, _, _) if audit_id != "me" => {
            event_with_id!(Create, Audit, AUDIT_ID => audit_id)
        }
        _ => None,
    }
}

pub fn infer_get_event(
    route: &RoutePattern,
) -> Option<(EventAction, ResourceType, Option<String>)> {
    match (
        route.get(0),
        route.get(1),
        route.get(2),
        route.get(3),
        route.get(4),
    ) {
        (Some("auth"), Some("confirm-email"), _, _, _) => event!(EmailConfirm, Auth),
        (Some("users"), Some("me"), _, _, _) => event!(Read, User),
        (Some("users"), Some(user_id), _, _, _) if user_id != "me" && user_id != "avatars" => {
            event_with_id!(Read, User, USER_ID => user_id)
        }
        (Some("teams"), Some("me"), _, _, _) => event!(Read, Team),
        (Some("teams"), Some(team_id), None, _, _)
            if team_id != "me" && team_id != "invitations" =>
        {
            event_with_id!(Read, Team, TEAM_ID => team_id)
        }
        (Some("teams"), Some(team_id), Some("company"), _, _) => {
            event_with_id!(Read, Company, TEAM_ID => team_id)
        }
        (Some("teams"), Some(team_id), Some("invitations"), _, _) => {
            event_with_id!(Read, Invitation, TEAM_ID => team_id)
        }
        // Audit routes
        (Some("audits"), Some("me"), _, _, _) => event!(Read, Audit),
        (Some("audits"), Some(audit_id), Some("board" | "messages"), _, _) if audit_id != "me" => {
            event_with_id!(Read, Audit, AUDIT_ID => audit_id)
        }
        (Some("audits"), Some(audit_id), Some("invite"), _, _)
            if audit_id != "me" && audit_id != "invite" =>
        {
            event_with_id!(Read, AuditInvitation, AUDIT_ID => audit_id)
        }
        (Some("audits"), Some(audit_id), Some("tickets"), _, _) if audit_id != "me" => {
            event_with_id!(Read, Ticket, AUDIT_ID => audit_id)
        }
        (Some("audits"), Some(audit_id), Some("columns"), _, _) if audit_id != "me" => {
            event_with_id!(Read, KanbanColumn, AUDIT_ID => audit_id)
        }
        (Some("audits"), Some(audit_id), Some("labels"), _, _) if audit_id != "me" => {
            event_with_id!(Read, TicketLabel, AUDIT_ID => audit_id)
        }
        // Agent routes
        (Some("audits"), Some(audit_id), Some("agents"), Some("download"), _) if audit_id != "me" => {
            event_with_id!(AgentDownload, Agent, AUDIT_ID => audit_id)
        }
        (Some("audits"), Some(audit_id), Some("agents"), _, _) if audit_id != "me" => {
            event_with_id!(Read, Agent, AUDIT_ID => audit_id)
        }
        _ => None,
    }
}

pub fn infer_patch_event(
    route: &RoutePattern,
) -> Option<(EventAction, ResourceType, Option<String>)> {
    match (
        route.get(0),
        route.get(1),
        route.get(2),
        route.get(3),
        route.get(4),
        route.get(5),
    ) {
        (Some("users"), Some("me"), _, _, _, _) => event!(Update, User),
        (Some("users"), Some(user_id), _, _, _, _) if user_id != "me" => {
            event_with_id!(Update, User, USER_ID => user_id)
        }
        (Some("teams"), Some(team_id), None, _, _, _) if team_id != "me" => {
            event_with_id!(Update, Team, TEAM_ID => team_id)
        }
        (Some("teams"), Some(team_id), Some("company"), Some(company_id), _, _) => {
            event_with_id!(Update, Company, TEAM_ID => team_id, COMPANY_ID => company_id)
        }
        (Some("teams"), Some(team_id), Some("members"), Some(user_id), _, _) => {
            event_with_id!(Update, TeamMember, TEAM_ID => team_id, USER_ID => user_id)
        }
        // Audit routes
        (Some("audits"), Some(audit_id), None, _, _, _) if audit_id != "me" => {
            event_with_id!(Update, Audit, AUDIT_ID => audit_id)
        }
        (Some("audits"), Some(audit_id), Some("tickets"), Some(ticket_id), None, _)
            if audit_id != "me" =>
        {
            event_with_id!(Update, Ticket, AUDIT_ID => audit_id, TICKET_ID => ticket_id)
        }
        (
            Some("audits"),
            Some(audit_id),
            Some("tickets"),
            Some(ticket_id),
            Some("comments"),
            Some(comment_id),
        ) if audit_id != "me" => {
            event_with_id!(Update, TicketComment, AUDIT_ID => audit_id, TICKET_ID => ticket_id, "comment_id" => comment_id)
        }
        (Some("audits"), Some(audit_id), Some("columns"), Some(column_id), _, _)
            if audit_id != "me" =>
        {
            event_with_id!(Update, KanbanColumn, AUDIT_ID => audit_id, COLUMN_ID => column_id)
        }
        (Some("audits"), Some(audit_id), Some("labels"), Some(label_id), _, _)
            if audit_id != "me" =>
        {
            event_with_id!(Update, TicketLabel, AUDIT_ID => audit_id, LABEL_ID => label_id)
        }
        _ => None,
    }
}

pub fn infer_delete_event(
    route: &RoutePattern,
) -> Option<(EventAction, ResourceType, Option<String>)> {
    match (
        route.get(0),
        route.get(1),
        route.get(2),
        route.get(3),
        route.get(4),
        route.get(5),
    ) {
        (Some("users"), Some("me"), Some("avatar"), _, _, _) => event!(AvatarDelete, Avatar),
        (Some("users"), Some("me"), None, _, _, _) => event!(Delete, User),
        (Some("users"), Some(user_id), _, _, _, _) if user_id != "me" => {
            event_with_id!(Delete, User, USER_ID => user_id)
        }
        (Some("teams"), Some(team_id), None, _, _, _) if team_id != "me" => {
            event_with_id!(Delete, Team, TEAM_ID => team_id)
        }
        (Some("teams"), Some(team_id), Some("company"), Some(company_id), _, _) => {
            event_with_id!(Delete, Company, TEAM_ID => team_id, COMPANY_ID => company_id)
        }
        (Some("teams"), Some(team_id), Some("members"), Some(user_id), _, _) => {
            event_with_id!(Delete, TeamMember, TEAM_ID => team_id, USER_ID => user_id)
        }
        (Some("teams"), Some(team_id), Some("invitations"), Some(token), _, _) => {
            event_with_id!(InvitationCancel, Invitation, TEAM_ID => team_id, TOKEN => token)
        }
        // Audit routes
        (Some("audits"), Some(audit_id), None, _, _, _) if audit_id != "me" => {
            event_with_id!(Delete, Audit, AUDIT_ID => audit_id)
        }
        (Some("audits"), Some(audit_id), Some("clients"), Some(user_id), _, _)
            if audit_id != "me" =>
        {
            event_with_id!(Delete, AuditClient, AUDIT_ID => audit_id, USER_ID => user_id)
        }
        (Some("audits"), Some(audit_id), Some("invite"), Some(invitation_id), _, _)
            if audit_id != "me" =>
        {
            event_with_id!(InvitationCancel, AuditInvitation, AUDIT_ID => audit_id, INVITATION_ID => invitation_id)
        }
        (Some("audits"), Some(audit_id), Some("tickets"), Some(ticket_id), None, _)
            if audit_id != "me" =>
        {
            event_with_id!(Delete, Ticket, AUDIT_ID => audit_id, TICKET_ID => ticket_id)
        }
        (
            Some("audits"),
            Some(audit_id),
            Some("tickets"),
            Some(ticket_id),
            Some("comments"),
            Some(comment_id),
        ) if audit_id != "me" => {
            event_with_id!(Delete, TicketComment, AUDIT_ID => audit_id, TICKET_ID => ticket_id, "comment_id" => comment_id)
        }
        (
            Some("audits"),
            Some(audit_id),
            Some("tickets"),
            Some(ticket_id),
            Some("attachments"),
            Some(attachment_id),
        ) if audit_id != "me" => {
            event_with_id!(Delete, TicketAttachment, AUDIT_ID => audit_id, TICKET_ID => ticket_id, "attachment_id" => attachment_id)
        }
        (
            Some("audits"),
            Some(audit_id),
            Some("tickets"),
            Some(ticket_id),
            Some("labels"),
            Some(label_id),
        ) if audit_id != "me" => {
            event_with_id!(Delete, TicketLabel, AUDIT_ID => audit_id, TICKET_ID => ticket_id, LABEL_ID => label_id)
        }
        (Some("audits"), Some(audit_id), Some("columns"), Some(column_id), _, _)
            if audit_id != "me" =>
        {
            event_with_id!(Delete, KanbanColumn, AUDIT_ID => audit_id, COLUMN_ID => column_id)
        }
        (Some("audits"), Some(audit_id), Some("labels"), Some(label_id), _, _)
            if audit_id != "me" =>
        {
            event_with_id!(Delete, TicketLabel, AUDIT_ID => audit_id, LABEL_ID => label_id)
        }
        // Agent revoke
        (Some("audits"), Some(audit_id), Some("agents"), Some(agent_id), _, _)
            if audit_id != "me" =>
        {
            event_with_id!(AgentShutdown, Agent, AUDIT_ID => audit_id, "agent_id" => agent_id)
        }
        _ => None,
    }
}
