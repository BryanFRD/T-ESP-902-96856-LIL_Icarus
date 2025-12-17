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
    role: &UserRole,
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
    role: &UserRole,
) -> User {
    let user = create_test_user(username, email, password, &role);
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
        &UserRole::Admin,
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
        &UserRole::Pentester,
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
        &UserRole::Client,
    )
}

// ============ Teams fixtures ============

#[cfg(test)]
use crate::api::teams::{Team, TeamMember, TeamRole};
#[cfg(test)]
use crate::schema::{teams, teams_members};

/// Crée une équipe de test et l'insère en base
#[cfg(test)]
pub fn insert_test_team(pool: &DbPool, name: &str, description: &str) -> Team {
    let team = Team::new(name.to_string(), description.to_string());
    let mut conn = pool.get().expect("Failed to get DB connection");

    diesel::insert_into(teams::table)
        .values(&team)
        .execute(&mut conn)
        .expect("Failed to insert test team");

    team
}

/// Ajoute un membre à une équipe avec un rôle spécifique
#[cfg(test)]
pub fn insert_team_member(
    pool: &DbPool,
    team_id: &str,
    user_id: &str,
    role: TeamRole,
) -> TeamMember {
    let member = TeamMember {
        team_id: team_id.to_string(),
        user_id: user_id.to_string(),
        role,
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };
    let mut conn = pool.get().expect("Failed to get DB connection");

    diesel::insert_into(teams_members::table)
        .values(&member)
        .execute(&mut conn)
        .expect("Failed to insert team member");

    member
}

/// Crée une équipe avec un owner
#[cfg(test)]
pub fn create_team_with_owner(pool: &DbPool, user: &User) -> Team {
    let team = insert_test_team(pool, "Test Team", "A test team");
    insert_team_member(pool, &team.id, &user.id, TeamRole::Owner);
    team
}

// ============ Companies fixtures ============

#[cfg(test)]
use crate::api::teams::companies::Company;
#[cfg(test)]
use crate::schema::companies;

/// Crée une entreprise de test et l'insère en base
#[cfg(test)]
pub fn insert_test_company(
    pool: &DbPool,
    team_id: &str,
    name: &str,
    description: Option<String>,
    siret: &str,
) -> Company {
    let company = Company::new(
        team_id.to_string(),
        name.to_string(),
        description,
        siret.to_string(),
    );
    let mut conn = pool.get().expect("Failed to get DB connection");

    diesel::insert_into(companies::table)
        .values(&company)
        .execute(&mut conn)
        .expect("Failed to insert test company");

    company
}
