use crate::api::events::{EventAction, ResourceType};

use super::route_pattern::RoutePattern;

const TEAM_ID: &str = "team_id";
const USER_ID: &str = "user_id";
const COMPANY_ID: &str = "company_id";
const TOKEN: &str = "token";

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
    match (route.get(0), route.get(1), route.get(2)) {
        (Some("auth"), Some("login"), _) => event!(Login, Auth),
        (Some("auth"), Some("register"), _) => event!(Register, Auth),
        (Some("auth"), Some("logout"), _) => event!(Logout, Auth),
        (Some("auth"), Some("reset-password"), _) => event!(PasswordReset, Auth),
        (Some("users"), Some("me"), Some("avatar")) => event!(AvatarUpload, Avatar),
        (Some("teams"), Some("me"), _) => event!(Create, Team),
        (Some("teams"), Some(team_id), Some("company")) => {
            event_with_id!(Create, Company, TEAM_ID => team_id)
        }
        (Some("teams"), Some(team_id), Some("invitations")) if team_id != "invitations" => {
            event_with_id!(InvitationCreate, Invitation, TEAM_ID => team_id)
        }
        (Some("teams"), Some("invitations"), Some("join")) => event!(InvitationConsume, Invitation),
        _ => None,
    }
}

pub fn infer_get_event(
    route: &RoutePattern,
) -> Option<(EventAction, ResourceType, Option<String>)> {
    match (route.get(0), route.get(1), route.get(2)) {
        (Some("auth"), Some("confirm-email"), _) => event!(EmailConfirm, Auth),
        (Some("users"), Some("me"), _) => event!(Read, User),
        (Some("users"), Some(user_id), _) if user_id != "me" && user_id != "avatars" => {
            event_with_id!(Read, User, USER_ID => user_id)
        }
        (Some("teams"), Some("me"), _) => event!(Read, Team),
        (Some("teams"), Some(team_id), None) if team_id != "me" && team_id != "invitations" => {
            event_with_id!(Read, Team, TEAM_ID => team_id)
        }
        (Some("teams"), Some(team_id), Some("company")) => {
            event_with_id!(Read, Company, TEAM_ID => team_id)
        }
        (Some("teams"), Some(team_id), Some("invitations")) => {
            event_with_id!(Read, Invitation, TEAM_ID => team_id)
        }
        _ => None,
    }
}

pub fn infer_patch_event(
    route: &RoutePattern,
) -> Option<(EventAction, ResourceType, Option<String>)> {
    match (route.get(0), route.get(1), route.get(2), route.get(3)) {
        (Some("users"), Some("me"), _, _) => event!(Update, User),
        (Some("users"), Some(user_id), _, _) if user_id != "me" => {
            event_with_id!(Update, User, USER_ID => user_id)
        }
        (Some("teams"), Some(team_id), None, _) if team_id != "me" => {
            event_with_id!(Update, Team, TEAM_ID => team_id)
        }
        (Some("teams"), Some(team_id), Some("company"), Some(company_id)) => {
            event_with_id!(Update, Company, TEAM_ID => team_id, COMPANY_ID => company_id)
        }
        (Some("teams"), Some(team_id), Some("members"), Some(user_id)) => {
            event_with_id!(Update, TeamMember, TEAM_ID => team_id, USER_ID => user_id)
        }
        _ => None,
    }
}

pub fn infer_delete_event(
    route: &RoutePattern,
) -> Option<(EventAction, ResourceType, Option<String>)> {
    match (route.get(0), route.get(1), route.get(2), route.get(3)) {
        (Some("users"), Some("me"), Some("avatar"), _) => event!(AvatarDelete, Avatar),
        (Some("users"), Some("me"), None, _) => event!(Delete, User),
        (Some("users"), Some(user_id), _, _) if user_id != "me" => {
            event_with_id!(Delete, User, USER_ID => user_id)
        }
        (Some("teams"), Some(team_id), None, _) if team_id != "me" => {
            event_with_id!(Delete, Team, TEAM_ID => team_id)
        }
        (Some("teams"), Some(team_id), Some("company"), Some(company_id)) => {
            event_with_id!(Delete, Company, TEAM_ID => team_id, COMPANY_ID => company_id)
        }
        (Some("teams"), Some(team_id), Some("members"), Some(user_id)) => {
            event_with_id!(Delete, TeamMember, TEAM_ID => team_id, USER_ID => user_id)
        }
        (Some("teams"), Some(team_id), Some("invitations"), Some(token)) => {
            event_with_id!(InvitationCancel, Invitation, TEAM_ID => team_id, TOKEN => token)
        }
        _ => None,
    }
}
