use crate::api::events::resource_fetcher::ResourceFetcher;
use crate::api::teams::Company;
use crate::schema::companies::dsl::{companies, id};
use crate::state::GlobalState;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use serde_json::Value as JsonValue;

pub struct CompanyFetcher;

impl ResourceFetcher for CompanyFetcher {
    fn fetch_as_json(&self, state: &GlobalState, resource_id: &str) -> Option<JsonValue> {
        let company_id = parse_resource_id(resource_id, "company_id");

        let company_id = if company_id == resource_id {
            if resource_id.starts_with("team_id:") && !resource_id.contains("company_id:") {
                return None;
            }
            resource_id
        } else {
            company_id
        };

        let mut conn = state.get_db_conn().ok()?;

        let company: Company = companies
            .filter(id.eq(company_id))
            .select(Company::as_select())
            .first(&mut conn)
            .ok()?;

        serde_json::to_value(&company).ok()
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
