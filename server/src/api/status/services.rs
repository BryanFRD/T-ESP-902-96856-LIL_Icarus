use axum::Router;

use crate::state::GlobalState;

use super::endpoints::get_status;

pub fn get_routes(state: GlobalState) -> axum::Router {
    Router::new()
        .route("/", axum::routing::get(get_status))
        .with_state(state)
}
