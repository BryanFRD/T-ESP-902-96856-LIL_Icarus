use crate::api::events::resource_fetcher::ResourceFetcher;
use crate::api::teams::TeamRole;
use crate::schema::teams_members::dsl::{
    role, team_id as tm_team_id, teams_members, user_id as tm_user_id,
};
use crate::state::GlobalState;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde_json::{Value as JsonValue, json};

pub struct TeamMemberFetcher;

impl ResourceFetcher for TeamMemberFetcher {
    fn fetch_as_json(&self, state: &GlobalState, resource_id: &str) -> Option<JsonValue> {
        let team_id = parse_resource_id(resource_id, "team_id");
        let user_id = parse_resource_id(resource_id, "user_id");

        if team_id == resource_id || user_id == resource_id {
            return None;
        }

        let mut conn = state.get_db_conn().ok()?;

        let member_role: TeamRole = teams_members
            .filter(tm_team_id.eq(team_id))
            .filter(tm_user_id.eq(user_id))
            .select(role)
            .first(&mut conn)
            .ok()?;

        let role_value = serde_json::to_value(&member_role).ok()?;

        Some(json!({
            "teamId": team_id,
            "userId": user_id,
            "role": role_value
        }))
    }
}

fn parse_resource_id<'a>(resource_id: &'a str, key: &str) -> &'a str {
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
