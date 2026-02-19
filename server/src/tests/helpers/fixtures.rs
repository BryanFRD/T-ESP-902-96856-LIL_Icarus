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
pub fn create_test_user(username: &str, email: &str, password: &str, role: &UserRole) -> User {
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
#[allow(dead_code)]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
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

// ============ Audits fixtures ============

#[cfg(test)]
use crate::api::audits::{Audit, AuditClient};
#[cfg(test)]
use crate::schema::{audits, audits_clients};

/// Crée un audit de test et l'insère en base
#[cfg(test)]
pub fn insert_test_audit(
    pool: &DbPool,
    name: &str,
    description: Option<String>,
    team_id: &str,
    company_id: &str,
    end_date: Option<chrono::NaiveDateTime>,
) -> Audit {
    let audit = Audit::new(
        name.to_string(),
        description,
        team_id.to_string(),
        company_id.to_string(),
        end_date,
    );
    let mut conn = pool.get().expect("Failed to get DB connection");

    diesel::insert_into(audits::table)
        .values(&audit)
        .execute(&mut conn)
        .expect("Failed to insert test audit");

    audit
}

/// Ajoute un client à un audit
#[cfg(test)]
pub fn insert_audit_client(pool: &DbPool, audit_id: &str, user_id: &str) -> AuditClient {
    let audit_client = AuditClient::new(audit_id.to_string(), user_id.to_string());
    let mut conn = pool.get().expect("Failed to get DB connection");

    diesel::insert_into(audits_clients::table)
        .values(&audit_client)
        .execute(&mut conn)
        .expect("Failed to insert audit client");

    audit_client
}

/// Crée un audit complet avec équipe, company et propriétaire
#[cfg(test)]
pub fn create_complete_audit_setup(pool: &DbPool, owner: &User) -> (Team, Company, Audit) {
    let team = create_team_with_owner(pool, owner);
    let company = insert_test_company(
        pool,
        &team.id,
        "Test Company",
        Some("A test company".to_string()),
        "12345678901234",
    );
    let audit = insert_test_audit(
        pool,
        "Test Audit",
        Some("A test audit".to_string()),
        &team.id,
        &company.id,
        None,
    );
    (team, company, audit)
}
// ============ Invitations fixtures ============

#[cfg(test)]
use crate::api::audits::invitations::AuditInvitation;
#[cfg(test)]
use crate::schema::audits_invitations;

/// Insère une invitation de test dans la base de données
#[cfg(test)]
pub fn insert_test_invitation(
    pool: &DbPool,
    audit_id: &str,
    client_email: &str,
    token: &str,
    creator_user_id: &str,
) -> AuditInvitation {
    let invitation = AuditInvitation::new(
        audit_id.to_string(),
        client_email.to_string(),
        token.to_string(),
        (chrono::Utc::now() + chrono::Duration::hours(24)).naive_utc(),
        creator_user_id.to_string(),
    );
    let mut conn = pool.get().expect("Failed to get DB connection");

    diesel::insert_into(audits_invitations::table)
        .values(&invitation)
        .execute(&mut conn)
        .expect("Failed to insert test invitation");

    invitation
}

/// Récupère une invitation du db par ID
#[cfg(test)]
pub fn get_test_invitation(pool: &DbPool, invitation_id: &str) -> Option<AuditInvitation> {
    let mut conn = pool.get().expect("Failed to get DB connection");
    audits_invitations::table
        .filter(audits_invitations::id.eq(invitation_id))
        .first::<AuditInvitation>(&mut conn)
        .ok()
}
