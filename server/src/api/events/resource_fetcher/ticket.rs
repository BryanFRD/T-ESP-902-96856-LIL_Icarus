use crate::api::events::resource_fetcher::{ResourceFetcher, parse_resource_id};
use crate::schema::tickets::dsl::{id, tickets};
use crate::state::GlobalState;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use serde_json::Value as JsonValue;

use crate::api::audits::tickets::models::ticket::Ticket;

pub struct TicketFetcher;

impl ResourceFetcher for TicketFetcher {
    fn fetch_as_json(&self, state: &GlobalState, resource_id: &str) -> Option<JsonValue> {
        let ticket_id = parse_resource_id(resource_id, "ticket_id");

        let mut conn = state.get_db_conn().ok()?;

        let ticket: Ticket = tickets
            .filter(id.eq(ticket_id))
            .select(Ticket::as_select())
            .first(&mut conn)
            .ok()?;

        serde_json::to_value(&ticket).ok()
    }
}
