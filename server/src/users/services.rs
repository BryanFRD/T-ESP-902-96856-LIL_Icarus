use crate::auth::{self, RegisterPayload, admin_middleware, hash_password};
use crate::schema::users::dsl::{users, id, username, first_name, last_name, email, password_hash, avatar_url};
use crate::state::GlobalState;
use crate::users::UserError;
use axum::extract::multipart::Multipart;
use axum::middleware::from_fn;
use axum::routing::{delete, get, patch, post};
use axum::{Router, middleware};
use diesel::dsl::update;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use image::{DynamicImage, GenericImageView};
use std::fs;
use std::path::Path;
use validator::Validate;

use super::User;
use super::endpoints::{serve_avatar, me, update_me, delete_me, upload_avatar, delete_avatar, list_users_admin, get_user_admin, update_user_admin, delete_user_admin};
use super::models::UpdatePayload;

pub fn get_routes(state: &GlobalState) -> Router {
    Router::new()
        .route("/avatars/{filename}", get(serve_avatar))
        .route("/me", get(me))
        .route("/me", patch(update_me))
        .route("/me", delete(delete_me))
        .route("/me/avatar", post(upload_avatar))
        .route("/me/avatar", delete(delete_avatar))
        .layer(from_fn(auth::middleware))
        .with_state(state.clone())
        .merge(get_admin_routes(state))
}

pub fn get_admin_routes(state: &GlobalState) -> Router {
    Router::new()
        .route("/", get(list_users_admin))
        .route("/{id}", get(get_user_admin))
        .route("/{id}", patch(update_user_admin))
        .route("/{id}", delete(delete_user_admin))
        .layer(middleware::from_fn(admin_middleware))
        .with_state(state.clone())
}
pub fn get_user_by_email(
    state: &GlobalState,
    query_email: &str,
) -> Result<Option<User>, UserError> {
    use diesel::prelude::*;

    let mut conn = match state.get_db_conn() {
        Ok(conn) => conn,
        Err(e) => return Err(UserError::DatabaseConnectionError(e)),
    };

    match users
        .filter(email.eq(query_email))
        .select(User::as_select())
        .first::<User>(&mut conn)
        .optional()
    {
        Ok(res) => Ok(res),
        Err(e) => Err(UserError::DatabaseError(e)),
    }
}

pub fn get_user_by_id(state: &GlobalState, user_id: &str) -> Result<Option<User>, UserError> {
    use diesel::prelude::*;

    let mut conn = state.get_db_conn()?;

    users
        .filter(id.eq(user_id))
        .select(User::as_select())
        .first::<User>(&mut conn)
        .optional()
        .map_err(|_| UserError::UserNotFound)
}

pub fn get_all_users(state: &GlobalState) -> Result<Vec<User>, UserError> {
    use diesel::prelude::*;

    let mut conn = state.get_db_conn()?;

    match users.select(User::as_select()).load::<User>(&mut conn) {
        Ok(user_list) => Ok(user_list),
        Err(e) => Err(UserError::DatabaseError(e)),
    }
}

pub fn create_user(state: &GlobalState, user: &User) -> Result<(), UserError> {
    use crate::schema::users;
    use diesel::prelude::*;

    let mut conn = state.get_db_conn()?;

    diesel::insert_into(users::table)
        .values(user)
        .execute(&mut conn)?;

    Ok(())
}

pub fn user_exists(
    state: &GlobalState,
    payload: &RegisterPayload,
) -> Result<Option<String>, UserError> {
    use diesel::dsl::exists;
    use diesel::select;

    let mut conn = state.get_db_conn()?;

    if select(exists(users.filter(email.eq(&payload.email)))).get_result(&mut conn)? {
        return Ok(Some("email".to_string()));
    }

    if select(exists(
        users
            .filter(first_name.eq(&payload.first_name))
            .filter(last_name.eq(&payload.last_name)),
    ))
    .get_result(&mut conn)?
    {
        return Ok(Some("name".to_string()));
    }

    if select(exists(users.filter(username.eq(&payload.username)))).get_result(&mut conn)? {
        return Ok(Some("username".to_string()));
    }

    Ok(None)
}

pub fn validate_update_payload(payload: &UpdatePayload) -> Result<(), UserError> {
    if payload.validate().is_err() {
        return Err(UserError::UpdatePayloadError("invalidEmail".to_string()));
    }

    if payload.username.is_none()
        && payload.first_name.is_none()
        && payload.last_name.is_none()
        && payload.email.is_none()
        && payload.password.is_none()
        && payload.new_password.is_none()
        && payload.confirm_password.is_none()
    {
        return Err(UserError::UpdatePayloadError("noFieldProvided".to_string()));
    }

    if let Some(ref new_password) = payload.new_password {
        if payload.password.is_none() {
            return Err(UserError::UpdatePayloadError("needOldPassword".to_string()));
        }
        match &payload.confirm_password {
            Some(confirm_pw) if confirm_pw == new_password => {}
            Some(_) => {
                return Err(UserError::UpdatePayloadError(
                    "passwordsDoNotMatch".to_string(),
                ));
            }
            None => {
                return Err(UserError::UpdatePayloadError(
                    "confirmationPasswordRequired".to_string(),
                ));
            }
        }
    }

    Ok(())
}

pub fn check_update_conflicts(
    state: &GlobalState,
    payload: &UpdatePayload,
    current_user: &User,
) -> Result<Option<String>, UserError> {
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
                return Ok(Some("email".to_string()));
            }
        }
        if let Some(ref new_username) = payload.username {
            if &db_username == new_username {
                return Ok(Some("username".to_string()));
            }
        }
        if let (Some(new_first), Some(new_last)) = (&payload.first_name, &payload.last_name) {
            if &db_first == new_first && &db_last == new_last {
                return Ok(Some("name".to_string()));
            }
        }
    }

    Ok(None)
}

pub fn update_user_fields(
    state: &GlobalState,
    user_id: &str,
    payload: &UpdatePayload,
) -> Result<(), UserError> {
    use diesel::prelude::*;

    let mut conn = state.get_db_conn()?;

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
            .map(|val| password_hash.eq(crate::auth::hash_password(val))),
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

pub async fn update_user_password(
    state: &GlobalState,
    user_id: &str,
    password: &str,
) -> Result<(), UserError> {
    use diesel::prelude::*;
    let mut conn = state.get_db_conn()?;

    diesel::update(users.filter(id.eq(user_id)))
        .set(password_hash.eq(hash_password(password)))
        .execute(&mut conn)?;

    Ok(())
}

pub fn delete_user(state: &GlobalState, user_id: &str) -> Result<usize, UserError> {
    use diesel::prelude::*;

    let mut conn = state.get_db_conn()?;

    match diesel::delete(users.filter(id.eq(user_id))).execute(&mut conn) {
        Ok(usize) => Ok(usize),
        Err(e) => Err(UserError::DatabaseError(e)),
    }
}

pub async fn save_avatar(
    state: &GlobalState,
    user_id: &str,
    mut multipart: Multipart,
) -> Result<String, UserError> {
    let avatar_dir = "dist/avatars";
    fs::create_dir_all(avatar_dir)?;

    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or("");

        if name != "avatar" {
            continue;
        }

        let content_type = field.content_type().unwrap_or("");
        match content_type {
            "image/jpeg" | "image/jpg" | "image/png" | "image/webp" | "image/gif" | "image/bmp" => {
            }
            _ => return Err(UserError::MultipartError("invalidFileType".to_string())),
        }

        let data = field.bytes().await?;

        if data.len() > 10 * 1024 * 1024 {
            return Err(UserError::MultipartError("fileTooLarge".to_string()));
        }

        let img = image::load_from_memory(&data)?;

        let resized_img = resize_image_if_needed(img, 512, 512);

        let webp_data = convert_to_webp(&resized_img, 80.0);

        if webp_data.len() > 2 * 1024 * 1024 {
            return Err(UserError::MultipartError("fileTooLarge".to_string()));
        }

        let ulid = ulid::Ulid::new();
        let filename = format!("{ulid}.webp");
        let file_path = format!("{avatar_dir}/{filename}");

        tokio::fs::write(&file_path, &webp_data).await?;

        // Mettre à jour l'utilisateur en base de données
        update_user_avatar(state, user_id, &filename)?;

        return Ok(filename);
    }

    Err(UserError::AvatarNotFound)
}

#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn resize_image_if_needed(img: DynamicImage, max_width: u32, max_height: u32) -> DynamicImage {
    let (width, height) = img.dimensions();

    if width <= max_width && height <= max_height {
        return img;
    }

    let ratio = (max_width as f32 / width as f32).min(max_height as f32 / height as f32);
    let new_width = (width as f32 * ratio) as u32;
    let new_height = (height as f32 * ratio) as u32;

    img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3)
}

fn convert_to_webp(
    img: &DynamicImage,
    quality: f32,
) -> Vec<u8> {
    let rgb_img = img.to_rgb8();
    let (width, height) = rgb_img.dimensions();

    let webp_encoder = webp::Encoder::from_rgb(&rgb_img, width, height);
    let webp_data = webp_encoder.encode(quality);

    webp_data.to_vec()
}

pub fn update_user_avatar(
    state: &GlobalState,
    user_id: &str,
    avatar_filename: &str,
) -> Result<(), diesel::result::Error> {
    use diesel::prelude::*;

    let mut conn = state
        .get_db_conn()
        .map_err(|_| diesel::result::Error::NotFound)?;

    let avatar_value = if avatar_filename.is_empty() {
        None
    } else {
        Some(avatar_filename.to_string())
    };

    diesel::update(users.filter(id.eq(user_id)))
        .set(avatar_url.eq(avatar_value))
        .execute(&mut conn)?;

    Ok(())
}

pub fn delete_avatar_file(filename: &str) {
    let file_path = format!("dist/avatars/{filename}");
    if Path::new(&file_path).exists() {
        if let Err(e) = fs::remove_file(&file_path) {
            tracing::error!("Failed to delete avatar file {}: {}", filename, e);
        }
    }
}
