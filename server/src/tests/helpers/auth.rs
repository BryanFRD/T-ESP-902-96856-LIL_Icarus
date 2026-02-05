#[cfg(test)]
use crate::api::auth::{EmailConfirmationToken, hash_password};
#[cfg(test)]
use crate::schema::password_reset_tokens;
#[cfg(test)]
use crate::tests::helpers::db::DbPool;
#[cfg(test)]
use chrono::NaiveDateTime;
#[cfg(test)]
use diesel::prelude::*;
#[cfg(test)]
use std::fmt::Write as FmtWrite;

/// Crée un token de confirmation email valide (non expiré)
#[cfg(test)]
pub fn create_valid_email_token(
    username: &str,
    first_name: &str,
    last_name: &str,
    email: &str,
    password: &str,
) -> String {
    let password_hash = hash_password(password);

    let token = EmailConfirmationToken::new(
        username.to_string(),
        first_name.to_string(),
        last_name.to_string(),
        email.to_string(),
        password_hash,
    );

    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or("test_jwt_secret_key_for_testing_only_12345".to_string());

    token
        .encrypt(jwt_secret.as_bytes())
        .expect("Failed to encrypt token")
}

/// Crée un token de confirmation email expiré
#[cfg(test)]
pub fn create_expired_email_token(
    username: &str,
    first_name: &str,
    last_name: &str,
    email: &str,
    password: &str,
) -> String {
    let password_hash = hash_password(password);

    // Créer un token avec une date d'expiration dans le passé (11 minutes - au-delà de l'expiration de 10 minutes)
    let expired_time = chrono::Utc::now() - chrono::Duration::minutes(11);

    let token = EmailConfirmationToken {
        username: username.to_string(),
        first_name: first_name.to_string(),
        last_name: last_name.to_string(),
        email: email.to_string(),
        password_hash,
        expires_at: expired_time,
    };

    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or("test_jwt_secret_key_for_testing_only_12345".to_string());

    token
        .encrypt(jwt_secret.as_bytes())
        .expect("Failed to encrypt token")
}

/// Structure pour représenter un token de reset password en base de données
#[cfg(test)]
#[derive(Insertable, Queryable)]
#[diesel(table_name = password_reset_tokens)]
struct PasswordResetToken {
    user_id: String,
    token_hash: String,
    created_at: NaiveDateTime,
}

/// Crée un token de reset password valide dans la base de données
/// Retourne le token non-hashé (celui que l'utilisateur recevrait par email)
#[cfg(test)]
pub fn create_valid_reset_token(pool: &DbPool, user_id: &str) -> String {
    // Générer un token aléatoire de 64 bytes en format hexa (comme dans le code de prod)
    let token_bytes: [u8; 64] = std::array::from_fn(|i| (i as u8).wrapping_mul(31));
    let token = token_bytes.iter().fold(String::new(), |mut output, b| {
        let _ = write!(output, "{b:02X}");
        output
    });

    // Hasher le token en utilisant la même fonction que le code de prod
    let token_hash = hash_password(&token);

    // Insérer dans la base de données
    let reset_token = PasswordResetToken {
        user_id: user_id.to_string(),
        token_hash,
        created_at: chrono::Local::now().naive_utc(),
    };

    let mut conn = pool.get().expect("Failed to get DB connection");

    diesel::insert_into(password_reset_tokens::table)
        .values(&reset_token)
        .on_conflict(password_reset_tokens::user_id)
        .do_update()
        .set((
            password_reset_tokens::token_hash.eq(&reset_token.token_hash),
            password_reset_tokens::created_at.eq(&reset_token.created_at),
        ))
        .execute(&mut conn)
        .expect("Failed to insert reset token");

    token
}

/// Crée un token de reset password expiré dans la base de données
/// Retourne le token non-hashé
#[cfg(test)]
pub fn create_expired_reset_token(pool: &DbPool, user_id: &str) -> String {
    // Générer un token aléatoire
    let token_bytes: [u8; 64] = std::array::from_fn(|i| (i as u8).wrapping_mul(37));
    let token = token_bytes.iter().fold(String::new(), |mut output, b| {
        let _ = write!(output, "{b:02X}");
        output
    });

    // Hasher le token en utilisant la même fonction que le code de prod
    let token_hash = hash_password(&token);

    // Créer un timestamp expiré (16 minutes dans le passé - au-delà de l'expiration de 15 minutes)
    let expired_time = chrono::Local::now().naive_utc() - chrono::Duration::minutes(16);

    let reset_token = PasswordResetToken {
        user_id: user_id.to_string(),
        token_hash,
        created_at: expired_time,
    };

    let mut conn = pool.get().expect("Failed to get DB connection");

    diesel::insert_into(password_reset_tokens::table)
        .values(&reset_token)
        .on_conflict(password_reset_tokens::user_id)
        .do_update()
        .set((
            password_reset_tokens::token_hash.eq(&reset_token.token_hash),
            password_reset_tokens::created_at.eq(&reset_token.created_at),
        ))
        .execute(&mut conn)
        .expect("Failed to insert expired reset token");

    token
}
