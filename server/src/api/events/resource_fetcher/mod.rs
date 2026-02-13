mod company;
mod team;
mod team_member;
mod user;

use crate::api::events::ResourceType;
use crate::state::GlobalState;
use serde_json::Value as JsonValue;

pub use company::CompanyFetcher;
pub use team::TeamFetcher;
pub use team_member::TeamMemberFetcher;
pub use user::UserFetcher;

pub fn parse_resource_id<'a>(resource_id: &'a str, key: &str) -> &'a str {
    if let Some(pos) = resource_id.find(&format!("{key}:")) {
        let start = pos + key.len() + 1;
        let end = resource_id[start..]
            .find(',')
            .map_or(resource_id.len(), |p| start + p);
        &resource_id[start..end]
    } else {
        resource_id
    }
}

pub trait ResourceFetcher: Send + Sync {
    fn fetch_as_json(&self, state: &GlobalState, resource_id: &str) -> Option<JsonValue>;
}

pub struct ResourceFetcherRegistry;

impl ResourceFetcherRegistry {
    #[must_use]
    pub fn get_fetcher(resource_type: &ResourceType) -> Option<Box<dyn ResourceFetcher>> {
        match resource_type {
            ResourceType::User => Some(Box::new(UserFetcher)),
            ResourceType::Team => Some(Box::new(TeamFetcher)),
            ResourceType::Company => Some(Box::new(CompanyFetcher)),
            ResourceType::TeamMember => Some(Box::new(TeamMemberFetcher)),
            ResourceType::Invitation
            | ResourceType::Auth
            | ResourceType::Avatar
            | ResourceType::Audit
            | ResourceType::AuditClient
            | ResourceType::AuditInvitation
            | ResourceType::Ticket
            | ResourceType::TicketComment
            | ResourceType::TicketAttachment
            | ResourceType::KanbanColumn
            | ResourceType::TicketLabel
            | ResourceType::Agent => None,
        }
    }

    #[must_use]
    pub fn fetch_before_state(
        state: &GlobalState,
        resource_type: &ResourceType,
        resource_id: Option<&String>,
    ) -> Option<JsonValue> {
        let fetcher = Self::get_fetcher(resource_type)?;
        let id = resource_id?;
        fetcher.fetch_as_json(state, id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_resource_id_single_key() {
        assert_eq!(parse_resource_id("team_id:123", "team_id"), "123");
    }

    #[test]
    fn parse_resource_id_multiple_keys() {
        assert_eq!(
            parse_resource_id("team_id:123,user_id:456", "team_id"),
            "123"
        );
        assert_eq!(
            parse_resource_id("team_id:123,user_id:456", "user_id"),
            "456"
        );
    }

    #[test]
    fn parse_resource_id_key_not_found_returns_full_id() {
        let resource_id = "team_id:123";
        assert_eq!(parse_resource_id(resource_id, "user_id"), resource_id);
    }

    #[test]
    fn parse_resource_id_uuid_format() {
        let resource_id = "team_id:550e8400-e29b-41d4-a716-446655440000,user_id:6ba7b810-9dad-11d1-80b4-00c04fd430c8";
        assert_eq!(
            parse_resource_id(resource_id, "team_id"),
            "550e8400-e29b-41d4-a716-446655440000"
        );
        assert_eq!(
            parse_resource_id(resource_id, "user_id"),
            "6ba7b810-9dad-11d1-80b4-00c04fd430c8"
        );
    }

    #[test]
    fn parse_resource_id_complex_format() {
        let resource_id = "team_id:123,user_id:456,company_id:789,role:admin";
        assert_eq!(parse_resource_id(resource_id, "team_id"), "123");
        assert_eq!(parse_resource_id(resource_id, "user_id"), "456");
        assert_eq!(parse_resource_id(resource_id, "company_id"), "789");
        assert_eq!(parse_resource_id(resource_id, "role"), "admin");
    }
}
