use crate::api::events::resource_fetcher::{ResourceFetcher, parse_resource_id};
use crate::api::users::User;
use crate::schema::users::dsl::{id, users};
use crate::state::GlobalState;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use serde_json::Value as JsonValue;

pub struct UserFetcher;

impl ResourceFetcher for UserFetcher {
    fn fetch_as_json(&self, state: &GlobalState, resource_id: &str) -> Option<JsonValue> {
        let user_id = parse_resource_id(resource_id, "user_id");

        let mut conn = state.get_db_conn().ok()?;

        let user: User = users
            .filter(id.eq(user_id))
            .select(User::as_select())
            .first(&mut conn)
            .ok()?;

        let mut json = serde_json::to_value(&user).ok()?;
        if let Some(obj) = json.as_object_mut() {
            obj.remove("passwordHash");
            obj.remove("password_hash");
        }

        Some(json)
    }
}
