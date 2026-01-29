use tracing::Level;

use crate::state::GlobalState;
use crate::vault::get_secrets;

pub fn start_logger() {
    tracing_subscriber::fmt()
        .with_target(true)
        .with_max_level(Level::INFO)
        .compact()
        .init();
}

pub async fn get_state() -> GlobalState {
    let secrets = match get_secrets().await {
        Ok(secrets) => secrets,
        Err(e) => {
            tracing::error!("Failed to get secrets: {}", e);
            panic!("Failed to get secrets");
        }
    };
    GlobalState::new(secrets)
}
