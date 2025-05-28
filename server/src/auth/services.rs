use super::{
    endpoints::{delete_me, login, logout, me, register, update_me},
    models::{RegisterPayload, UpdatePayload},
};
use axum::{
    Router,
    routing::{delete, get, patch, post},
};
use diesel::dsl::update;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use sha2::{Digest, Sha256};
use tracing::error;
use validator::Validate;

use super::User;
use crate::state::GlobalState;

pub fn get_no_auth_routes(state: GlobalState) -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .with_state(state)
}

pub fn get_routes(state: GlobalState) -> Router {
    Router::new()
        .route("/me", get(me))
        .route("/me", patch(update_me))
        .route("/me", delete(delete_me))
        .route("/logout", post(logout))
        .with_state(state)
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

pub fn get_user_by_id(
    state: &GlobalState,
    user_id: &str,
) -> Result<Option<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    use diesel::prelude::*;

    let mut conn = match state.get_db_conn() {
        Ok(conn) => conn,
        Err(_) => return Err(diesel::result::Error::NotFound),
    };

    users
        .filter(id.eq(user_id))
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

pub fn validate_update_payload(payload: &UpdatePayload) -> Result<(), String> {
    payload
        .validate()
        .map_err(|_| "Invalid update payload".to_string())?;

    if payload.username.is_none()
        && payload.first_name.is_none()
        && payload.last_name.is_none()
        && payload.email.is_none()
        && payload.password.is_none()
        && payload.new_password.is_none()
        && payload.confirm_password.is_none()
    {
        return Err("No fields to update".to_string());
    }

    if let Some(ref new_password) = payload.new_password {
        if payload.password.is_none() {
            return Err("Current password is required to set a new password".to_string());
        }
        match &payload.confirm_password {
            Some(confirm_pw) if confirm_pw == new_password => {}
            Some(_) => return Err("New password and confirmation do not match".to_string()),
            None => return Err("Confirmation password is required".to_string()),
        }
    }

    Ok(())
}

pub fn check_update_conflicts(
    state: &GlobalState,
    payload: &UpdatePayload,
    current_user: &User,
) -> Result<Option<String>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    use diesel::prelude::*;

    let mut conn = state
        .get_db_conn()
        .map_err(|_| diesel::result::Error::NotFound)?;

    let mut query = users.filter(id.ne(&current_user.id)).into_boxed();

    let mut has_condition = false;

    if let Some(ref new_email) = payload.email {
        query = query.or_filter(email.eq(new_email));
        has_condition = true;
    }
    if let Some(ref new_username) = payload.username {
        query = query.or_filter(username.eq(new_username));
        has_condition = true;
    }
    if let (Some(new_first), Some(new_last)) = (&payload.first_name, &payload.last_name) {
        query = query.or_filter(first_name.eq(new_first).and(last_name.eq(new_last)));
        has_condition = true;
    }

    if !has_condition {
        return Ok(None);
    }

    let existing = query
        .select((email, username, first_name, last_name))
        .first::<(String, String, String, String)>(&mut conn)
        .optional()?;

    if let Some((db_email, db_username, db_first, db_last)) = existing {
        if let Some(ref new_email) = payload.email {
            if &db_email == new_email {
                return Ok(Some("Email already exists".to_string()));
            }
        }
        if let Some(ref new_username) = payload.username {
            if &db_username == new_username {
                return Ok(Some("Username already exists".to_string()));
            }
        }
        if let (Some(new_first), Some(new_last)) = (&payload.first_name, &payload.last_name) {
            if &db_first == new_first && &db_last == new_last {
                return Ok(Some("Name already exists".to_string()));
            }
        }
    }

    Ok(None)
}

use crate::schema::users::dsl::*;

pub fn update_user_fields(
    state: &GlobalState,
    user_id: &str,
    payload: &UpdatePayload,
) -> Result<(), diesel::result::Error> {
    use diesel::prelude::*;

    let mut conn = state
        .get_db_conn()
        .map_err(|_| diesel::result::Error::NotFound)?;

    let changes = update(users.filter(id.eq(user_id)));

    // Build a tuple of Option fields for updating
    let update_fields = (
        payload.username.as_ref().map(|val| username.eq(val)),
        payload.first_name.as_ref().map(|val| first_name.eq(val)),
        payload.last_name.as_ref().map(|val| last_name.eq(val)),
        payload.email.as_ref().map(|val| email.eq(val)),
        payload
            .new_password
            .as_ref()
            .map(|val| password_hash.eq(crate::auth::services::hash_password(val))),
    );

    // Only update if there is at least one field to set
    if update_fields.0.is_some()
        || update_fields.1.is_some()
        || update_fields.2.is_some()
        || update_fields.3.is_some()
        || update_fields.4.is_some()
    {
        changes
            .set((
                update_fields.0,
                update_fields.1,
                update_fields.2,
                update_fields.3,
                update_fields.4,
            ))
            .execute(&mut conn)?;
    }

    Ok(())
}

pub fn delete_user(state: &GlobalState, user_id: &str) -> Result<(), diesel::result::Error> {
    use crate::schema::users::dsl::*;
    use diesel::prelude::*;

    let mut conn = state
        .get_db_conn()
        .map_err(|_| diesel::result::Error::NotFound)?;

    diesel::delete(users.filter(id.eq(user_id)))
        .execute(&mut conn)
        .map(|_| ())
}
