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
            ResourceType::Invitation | ResourceType::Auth | ResourceType::Avatar => None,
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
