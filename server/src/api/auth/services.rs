use super::endpoints::{confirm_email, login, logout, register, request_password_reset};
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{Router, middleware::from_fn, routing::post};
use diesel::{
    dsl::{IntervalDsl, now},
    upsert::excluded,
};
use sha2::{Digest, Sha256};

use crate::api::{
    auth::{
        self,
        endpoints::reset_password,
        models::{AddPasswordResetToken, PasswordResetToken},
    },
    users::UserError,
};
use crate::state::GlobalState;

pub fn get_routes(state: GlobalState) -> Router {
    Router::new()
        .route("/logout", post(logout))
        .layer(from_fn(auth::middleware))
        .route("/request-password-reset", post(request_password_reset))
        .route("/reset-password", post(reset_password))
        .route("/confirm-email", post(confirm_email))
        .route("/login", post(login))
        .route("/register", post(register))
        .with_state(state)
}

/// Legacy double SHA-256 password hashing (version 1)
///
/// # Security Warning
/// This function is deprecated and should only be used for verifying legacy passwords.
/// New passwords should use `hash_password_argon2id()`.
pub fn hash_password_sha256(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    let first_hash = hasher.finalize();
    let first_hash_hex = format!("{first_hash:x}");

    let mut hasher2 = Sha256::new();
    hasher2.update(format!("{password}{first_hash_hex}"));
    let result = hasher2.finalize();
    format!("{result:x}")
}

/// Hash password using Argon2id (version 2)
///
/// Parameters:
/// - `m_cost`: 19456 KiB (19 MiB) - default
/// - `t_cost`: 2 iterations - default
/// - `p_cost`: 1 thread - default
/// - `output_len`: 32 bytes - default
///
/// Returns PHC-formatted string: $argon2id$v=19$m=19456,t=2,p=1$<salt>$<hash>
pub fn hash_password_argon2id(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string()
}

/// Backward compatibility wrapper - delegates to SHA-256
///
/// # Deprecated
/// Use `hash_password_argon2id` for new passwords.
/// This function exists only for backward compatibility with existing code.
#[allow(dead_code)]
pub fn hash_password(password: &str) -> String {
    hash_password_sha256(password)
}

/// Verify password with automatic migration detection
///
/// Version handling:
/// - None or Some(1): Use SHA-256 verification (legacy)
/// - Some(2): Use Argon2id verification
///
/// Returns true if password matches the stored hash
pub fn verify_password_with_migration(
    password: &str,
    stored_hash: &str,
    version: Option<i16>,
) -> bool {
    match version {
        Some(2) => {
            // Argon2id verification
            if let Ok(parsed_hash) = PasswordHash::new(stored_hash) {
                Argon2::default()
                    .verify_password(password.as_bytes(), &parsed_hash)
                    .is_ok()
            } else {
                false
            }
        }
        None | Some(1) => {
            // Legacy SHA-256 verification
            let computed_hash = hash_password_sha256(password);
            computed_hash == stored_hash
        }
        Some(v) => {
            tracing::error!("Unknown password hash version: {}", v);
            false
        }
    }
}

pub async fn save_reset_password_token(
    state: &GlobalState,
    user_id: &str,
    token: &str,
) -> Result<(), UserError> {
    use crate::schema::password_reset_tokens;
    use diesel::prelude::*;

    let mut conn = state.get_db_conn()?;

    diesel::insert_into(password_reset_tokens::table)
        .values(&AddPasswordResetToken {
            user_id: user_id.to_string(),
            // Note: Reset tokens continue using SHA-256 for simplicity
            // as they are short-lived (15 minutes) and single-use
            token_hash: hash_password_sha256(token),
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

pub async fn get_token(
    state: &GlobalState,
    token: &str,
) -> Result<Option<PasswordResetToken>, UserError> {
    use crate::schema::password_reset_tokens::dsl::{password_reset_tokens, token_hash};
    use diesel::prelude::*;

    let mut conn = state.get_db_conn()?;

    match password_reset_tokens
        .filter(token_hash.eq(&hash_password_sha256(token)))
        .select(PasswordResetToken::as_select())
        .first::<PasswordResetToken>(&mut conn)
        .optional()
    {
        Ok(res) => Ok(res),
        Err(e) => Err(UserError::DatabaseError(e)),
    }
}

pub async fn consume_token(state: &GlobalState, id: &str) -> Result<(), UserError> {
    use crate::schema::password_reset_tokens::dsl::{created_at, password_reset_tokens, user_id};
    use diesel::prelude::*;

    let mut conn = state.get_db_conn()?;
    diesel::update(password_reset_tokens.filter(user_id.eq(id)))
        .set(created_at.eq(now - 15.minutes()))
        .execute(&mut conn)?;
    Ok(())
}
