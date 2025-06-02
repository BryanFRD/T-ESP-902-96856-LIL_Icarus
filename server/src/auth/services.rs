use super::endpoints::{login, logout, register};
use axum::{Router, routing::post};
use sha2::{Digest, Sha256};

use crate::state::GlobalState;

pub fn get_no_auth_routes(state: GlobalState) -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .with_state(state)
}

pub fn get_routes(state: GlobalState) -> Router {
    Router::new()
        .route("/logout", post(logout))
        .with_state(state)
}

pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    let first_hash = hasher.finalize();
    let first_hash_hex = format!("{:x}", first_hash);

    let mut hasher2 = Sha256::new();
    hasher2.update(format!("{}{}", password, first_hash_hex));
    let result = hasher2.finalize();
    format!("{:x}", result)
}
