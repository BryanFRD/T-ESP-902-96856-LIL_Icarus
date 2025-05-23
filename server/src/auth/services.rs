use super::{
    endpoints::{login, me, register},
    models::RegisterPayload,
};
use axum::{
    Router,
    routing::{get, post},
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use sha2::{Digest, Sha256};
use tracing::error;

use super::User;
use crate::state::GlobalState;

pub fn get_no_auth_routes(state: GlobalState) -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .with_state(state)
}

pub fn get_routes(state: GlobalState) -> Router {
    Router::new().route("/me", get(me)).with_state(state)
}

pub fn get_user_by_email(
    state: &GlobalState,
    query_email: &str,
) -> Result<Option<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    use diesel::prelude::*;

    let mut conn = match state.get_db_conn() {
        Ok(conn) => conn,
        Err(_) => return Err(diesel::result::Error::NotFound),
    };

    users
        .filter(email.eq(query_email))
        .select(User::as_select())
        .first::<User>(&mut conn)
        .optional()
        .map_err(|_| diesel::result::Error::NotFound)
}

pub fn create_user(state: &GlobalState, user: User) -> Result<(), diesel::result::Error> {
    use crate::schema::users;
    use diesel::prelude::*;

    let mut conn = match state.get_db_conn() {
        Ok(conn) => conn,
        Err(_) => return Err(diesel::result::Error::NotFound),
    };

    diesel::insert_into(users::table)
        .values(&user)
        .execute(&mut conn)?;

    Ok(())
}

pub fn user_exists(
    state: &GlobalState,
    payload: &RegisterPayload,
) -> Result<Option<String>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    use diesel::dsl::exists;
    use diesel::select;

    let mut conn = match state.get_db_conn() {
        Ok(conn) => conn,
        Err(e) => {
            error!("Failed to get DB connection: {}", e);
            return Err(diesel::result::Error::NotFound);
        }
    };

    if select(exists(users.filter(email.eq(&payload.email)))).get_result(&mut conn)? {
        return Ok(Some("Email already exists".to_string()));
    }

    if select(exists(
        users
            .filter(first_name.eq(&payload.first_name))
            .filter(last_name.eq(&payload.last_name)),
    ))
    .get_result(&mut conn)?
    {
        return Ok(Some("Name already exists".to_string()));
    }

    if select(exists(users.filter(username.eq(&payload.username)))).get_result(&mut conn)? {
        return Ok(Some("Username already exists".to_string()));
    }

    Ok(None)
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
