use super::endpoints::{login, logout, register, request_password_reset};
use axum::{middleware::from_fn, routing::post, Router};
use diesel::{dsl::{now, IntervalDsl}, upsert::excluded};
use sha2::{Digest, Sha256};

use crate::{auth::{self, endpoints::reset_password, models::{AddPasswordResetToken, PasswordResetToken}}, state::GlobalState, users::UserError};

pub fn get_routes(state: GlobalState) -> Router {
    Router::new()
        .route("/logout", post(logout))
        .layer(from_fn(auth::middleware))
        .route("/request-password-reset", post(request_password_reset))
        .route("/reset-password", post(reset_password))
        .route("/login", post(login))
        .route("/register", post(register))
        .with_state(state)
}

pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    let first_hash = hasher.finalize();
    let first_hash_hex = format!("{first_hash:x}");

    let mut hasher2 = Sha256::new();
    hasher2.update(format!("{password}{first_hash_hex}"));
    let result = hasher2.finalize();
    format!("{result:x}")
}

pub async fn save_reset_password_token(state: &GlobalState, user_id: &str, token: &str) -> Result<(), UserError> {
    use crate::schema::password_reset_tokens;
    use diesel::prelude::*;

    let mut conn = state.get_db_conn()?;

    diesel::insert_into(password_reset_tokens::table)
        .values(&AddPasswordResetToken {
            user_id: user_id.to_string(),
            token_hash: hash_password(token),
        })
        .on_conflict(password_reset_tokens::user_id)
        .do_update()
        .set((
            password_reset_tokens::token_hash.eq(excluded(password_reset_tokens::token_hash)),
            password_reset_tokens::created_at.eq(now),
        ))
        .execute(&mut conn)?;

    Ok(())
}

pub async fn get_token(state: &GlobalState, token: &str) -> Result<Option<PasswordResetToken>, UserError> {
    use crate::schema::password_reset_tokens::dsl::*;
    use diesel::prelude::*;

    let mut conn = state.get_db_conn()?;

    match password_reset_tokens
        .filter(token_hash.eq(&hash_password(token)))
        .select(PasswordResetToken::as_select())
        .first::<PasswordResetToken>(&mut conn)
        .optional() {
            Ok(res) => Ok(res),
            Err(e) => Err(UserError::DatabaseError(e))
    }
}

pub async fn consume_token(state: &GlobalState, id: &str) -> Result<(), UserError> {
    use crate::schema::password_reset_tokens::dsl::*;
    use diesel::prelude::*;

    let mut conn = state.get_db_conn()?;
    diesel::update(password_reset_tokens.filter(user_id.eq(id)))
        .set(created_at.eq(now - 15.minutes()))
        .execute(&mut conn)?;
    Ok(())
}
