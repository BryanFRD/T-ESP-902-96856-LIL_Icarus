#[cfg(test)]
use crate::api::users::{User, UserRole};
#[cfg(test)]
use crate::schema::users;
#[cfg(test)]
use crate::tests::helpers::db::DbPool;
#[cfg(test)]
use diesel::prelude::*;

/// Crée un utilisateur de test avec des valeurs par défaut
/// Retourne l'utilisateur créé (pas encore inséré en base)
#[cfg(test)]
pub fn create_test_user(
    username: &str,
    email: &str,
    password: &str,
    role: UserRole,
) -> User {
    User::new(
        username.to_string(),
        "Test".to_string(),
        "User".to_string(),
        email.to_string(),
        match role {
            UserRole::Admin => "admin".to_string(),
            UserRole::Pentester => "pentester".to_string(),
            UserRole::Client => "client".to_string(),
        },
        password,
    )
}

/// Insère un utilisateur de test dans la base de données
/// Retourne l'utilisateur inséré
#[cfg(test)]
pub fn insert_test_user(
    pool: &DbPool,
    username: &str,
    email: &str,
    password: &str,
    role: UserRole,
) -> User {
    let user = create_test_user(username, email, password, role);
    let mut conn = pool.get().expect("Failed to get DB connection");

    diesel::insert_into(users::table)
        .values(&user)
        .execute(&mut conn)
        .expect("Failed to insert test user");

    user
}

/// Crée un utilisateur admin de test et l'insère en base
#[cfg(test)]
pub fn create_admin_user(pool: &DbPool) -> User {
    insert_test_user(
        pool,
        "admin_test",
        "admin@test.com",
        "admin_password",
        UserRole::Admin,
    )
}

/// Crée un utilisateur pentester de test et l'insère en base
#[cfg(test)]
pub fn create_pentester_user(pool: &DbPool) -> User {
    insert_test_user(
        pool,
        "pentester_test",
        "pentester@test.com",
        "pentester_password",
        UserRole::Pentester,
    )
}

/// Crée un utilisateur client de test et l'insère en base
#[cfg(test)]
pub fn create_client_user(pool: &DbPool) -> User {
    insert_test_user(
        pool,
        "client_test",
        "client@test.com",
        "client_password",
        UserRole::Client,
    )
}
