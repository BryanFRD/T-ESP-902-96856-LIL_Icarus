use crate::api::events::resource_fetcher::{ResourceFetcher, parse_resource_id};
use crate::api::teams::Team;
use crate::schema::teams::dsl::{id, teams};
use crate::state::GlobalState;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use serde_json::Value as JsonValue;

pub struct TeamFetcher;

impl ResourceFetcher for TeamFetcher {
    fn fetch_as_json(&self, state: &GlobalState, resource_id: &str) -> Option<JsonValue> {
        let team_id = parse_resource_id(resource_id, "team_id");

        let mut conn = state.get_db_conn().ok()?;

        let team: Team = teams
            .filter(id.eq(team_id))
            .select(Team::as_select())
            .first(&mut conn)
            .ok()?;

        serde_json::to_value(&team).ok()
    }
}
