use super::{
    Audit, AuditClient, AuditError, AuditResponse, CreateAuditPayload, UpdateAuditPayload,
};
use crate::{
    api::{
        teams::{companies::models::Company, does_team_exist, is_team_owner},
        users::User,
    },
    middlewares::{audit_step_up_middleware, auth},
    state::GlobalState,
};
use axum::{
    Router,
    middleware::{from_fn, from_fn_with_state},
    routing::{delete, get, patch, post},
};
use diesel::{
    ExpressionMethods, JoinOnDsl, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
};

use super::{endpoints, invitations, step_up};

/// Get routes for audits module
pub fn get_routes(state: &GlobalState) -> Router {
    // Routes protected by step-up authentication (all existing audit routes)
    let protected_routes = Router::new()
        .route("/me", get(endpoints::list_my_audits))
        .route("/me", post(endpoints::create_audit))
        .route("/{audit_id}", patch(endpoints::update_audit))
        .route("/{audit_id}", delete(endpoints::archive_audit))
        .route(
            "/{audit_id}/clients/{user_id}",
            delete(endpoints::remove_client),
        )
        .layer(from_fn_with_state(state.clone(), audit_step_up_middleware))
        .layer(from_fn(auth::middleware));

    // Routes for invitations (protected by auth but not step-up)
    let invitation_routes = Router::new()
        .route(
            "/{audit_id}/invite",
            post(invitations::endpoints::create_invitation),
        )
        .route(
            "/{audit_id}/invite",
            get(invitations::endpoints::list_invitations),
        )
        .route(
            "/{audit_id}/invite/{invitation_id}",
            delete(invitations::endpoints::cancel_invitation),
        )
        .layer(from_fn(auth::middleware));

    // Routes for verification (not protected by step-up)
    let verify_routes = Router::new()
        .route("/verify-access", post(step_up::endpoints::verify_access))
        .route("/check-access", get(step_up::endpoints::check_access))
        .layer(from_fn(auth::middleware));

    // Public/semi-public routes (no auth required for check, optional for accept)
    let public_routes = Router::new()
        .route(
            "/invite/{token}/check",
            get(invitations::endpoints::check_invitation),
        )
        .route(
            "/invite/{token}/accept",
            post(invitations::endpoints::accept_invitation),
        );

    // Merge all routers
    protected_routes
        .merge(invitation_routes)
        .merge(verify_routes)
        .merge(public_routes)
        .with_state(state.clone())
}

/// Helper: Get audit by ID
///
/// # Errors
/// Returns `AuditError` if audit not found or database error
pub fn get_audit(state: &GlobalState, audit_id: &str) -> Result<Audit, AuditError> {
    use crate::schema::audits;

    let mut conn = state.get_db_conn()?;
    let audit = audits::table
        .filter(audits::id.eq(audit_id))
        .select(Audit::as_select())
        .first::<Audit>(&mut conn)
        .optional()?
        .ok_or(AuditError::NotFound)?;

    Ok(audit)
}

/// Helper: Auto-archive audit if `end_date` passed
///
/// # Errors
/// Returns `AuditError` if database error
pub fn auto_archive_if_needed(state: &GlobalState, audit: &mut Audit) -> Result<bool, AuditError> {
    if audit.should_auto_archive() {
        use crate::schema::audits;

        let mut conn = state.get_db_conn()?;
        diesel::update(audits::table.filter(audits::id.eq(&audit.id)))
            .set(audits::is_archived.eq(true))
            .execute(&mut conn)?;

        audit.is_archived = true;
        return Ok(true);
    }
    Ok(false)
}

/// Helper: Check if user can modify audit (is team owner or admin)
///
/// # Errors
/// Returns `AuditError` if user not authorized
pub fn can_modify_audit(
    state: &GlobalState,
    audit: &Audit,
    user_id: &str,
    is_admin: bool,
) -> Result<bool, AuditError> {
    if is_admin {
        return Ok(true);
    }

    Ok(is_team_owner(state, &audit.team_id, user_id)?)
}

/// Helper: Validate company belongs to team
fn validate_company_belongs_to_team(
    state: &GlobalState,
    team_id: &str,
    company_id: &str,
) -> Result<(), AuditError> {
    use crate::schema::companies;

    let mut conn = state.get_db_conn()?;
    let company = companies::table
        .filter(companies::id.eq(company_id))
        .select(companies::team_id)
        .first::<String>(&mut conn)
        .optional()?
        .ok_or(AuditError::CompanyNotFound)?;

    if company != team_id {
        return Err(AuditError::CompanyTeamMismatch);
    }

    Ok(())
}

/// Helper: Validate `end_date` is not in the past
fn validate_end_date(end_date: &chrono::NaiveDateTime) -> Result<(), AuditError> {
    let now = chrono::Utc::now().naive_utc();
    if *end_date < now {
        return Err(AuditError::EndDateInPast);
    }
    Ok(())
}

/// Helper: Get all team IDs a user belongs to
fn get_user_team_ids(state: &GlobalState, user_id: &str) -> Result<Vec<String>, AuditError> {
    use crate::schema::teams_members;

    let mut conn = state.get_db_conn()?;
    let team_ids = teams_members::table
        .filter(teams_members::user_id.eq(user_id))
        .select(teams_members::team_id)
        .load::<String>(&mut conn)?;

    Ok(team_ids)
}

/// Helper: Check if client is in audit
fn is_client_in_audit(
    state: &GlobalState,
    audit_id: &str,
    user_id: &str,
) -> Result<bool, AuditError> {
    use crate::schema::audits_clients;

    let mut conn = state.get_db_conn()?;
    let exists = audits_clients::table
        .filter(audits_clients::audit_id.eq(audit_id))
        .filter(audits_clients::user_id.eq(user_id))
        .count()
        .get_result::<i64>(&mut conn)?
        > 0;

    Ok(exists)
}

/// List all audits for current user (based on team membership)
///
/// # Errors
/// Returns `AuditError` if database error or user not found
pub fn list_user_audits(
    state: &GlobalState,
    user_id: &str,
) -> Result<Vec<AuditResponse>, AuditError> {
    use crate::schema::{audits, audits_clients, companies, users};

    let team_ids = get_user_team_ids(state, user_id)?;

    if team_ids.is_empty() {
        return Ok(Vec::new());
    }

    let mut conn = state.get_db_conn()?;

    // Batch archive expired audits for performance
    let now = chrono::Utc::now().naive_utc();
    diesel::update(audits::table)
        .filter(audits::team_id.eq_any(&team_ids))
        .filter(audits::end_date.lt(now))
        .filter(audits::is_archived.eq(false))
        .set(audits::is_archived.eq(true))
        .execute(&mut conn)?;

    let audits_with_companies: Vec<(Audit, Company)> = audits::table
        .filter(audits::team_id.eq_any(&team_ids))
        .inner_join(companies::table.on(audits::company_id.eq(companies::id)))
        .select((Audit::as_select(), Company::as_select()))
        .load::<(Audit, Company)>(&mut conn)?;

    let audit_ids: Vec<String> = audits_with_companies
        .iter()
        .map(|(a, _)| a.id.clone())
        .collect();

    if audit_ids.is_empty() {
        return Ok(Vec::new());
    }

    let audit_clients: Vec<(AuditClient, User)> = audits_clients::table
        .filter(audits_clients::audit_id.eq_any(&audit_ids))
        .inner_join(users::table.on(audits_clients::user_id.eq(users::id)))
        .select((AuditClient::as_select(), User::as_select()))
        .load::<(AuditClient, User)>(&mut conn)?;

    let mut clients_map: std::collections::HashMap<String, Vec<User>> =
        std::collections::HashMap::new();
    for (ac, user) in audit_clients {
        clients_map.entry(ac.audit_id).or_default().push(user);
    }

    let result = audits_with_companies
        .into_iter()
        .map(|(audit, company)| {
            let clients = clients_map.remove(&audit.id).unwrap_or_default();
            AuditResponse::from_audit_and_clients(audit, company.into(), clients)
        })
        .collect();

    Ok(result)
}

/// Create a new audit
///
/// # Errors
/// Returns `AuditError` if validation fails or database error
pub fn create_audit(
    state: &GlobalState,
    user_id: &str,
    payload: CreateAuditPayload,
    is_admin: bool,
) -> Result<AuditResponse, AuditError> {
    use crate::schema::{audits, companies};

    if !does_team_exist(state, &payload.team_id)? {
        return Err(AuditError::TeamNotFound);
    }

    if !is_admin && !is_team_owner(state, &payload.team_id, user_id)? {
        return Err(AuditError::MustBeOwnerOrAdmin);
    }

    validate_company_belongs_to_team(state, &payload.team_id, &payload.company_id)?;

    if let Some(end_date) = &payload.end_date {
        validate_end_date(end_date)?;
    }

    let new_audit = Audit::new(
        payload.name,
        payload.description,
        payload.team_id,
        payload.company_id.clone(),
        payload.end_date,
    );

    let mut conn = state.get_db_conn()?;

    diesel::insert_into(audits::table)
        .values(&new_audit)
        .execute(&mut conn)?;

    let company = companies::table
        .filter(companies::id.eq(&payload.company_id))
        .select(Company::as_select())
        .first::<Company>(&mut conn)?;

    Ok(AuditResponse::from_audit_and_clients(
        new_audit,
        company.into(),
        Vec::new(),
    ))
}

/// Update an audit
///
/// # Errors
/// Returns `AuditError` if validation fails or database error
pub fn update_audit(
    state: &GlobalState,
    audit_id: &str,
    user_id: &str,
    payload: &UpdateAuditPayload,
    is_admin: bool,
) -> Result<AuditResponse, AuditError> {
    use crate::schema::{audits, audits_clients, companies, users};

    let mut audit = get_audit(state, audit_id)?;
    auto_archive_if_needed(state, &mut audit)?;

    if audit.is_archived {
        return Err(AuditError::AuditArchived);
    }

    if !can_modify_audit(state, &audit, user_id, is_admin)? {
        return Err(AuditError::MustBeOwnerOrAdmin);
    }

    if let Some(end_date) = &payload.end_date {
        validate_end_date(end_date)?;
    }

    let mut conn = state.get_db_conn()?;

    if let Some(name) = &payload.name {
        diesel::update(audits::table.filter(audits::id.eq(audit_id)))
            .set(audits::name.eq(name))
            .execute(&mut conn)?;
    }

    if let Some(description) = &payload.description {
        diesel::update(audits::table.filter(audits::id.eq(audit_id)))
            .set(audits::description.eq(description))
            .execute(&mut conn)?;
    }

    if payload.clear_end_date {
        diesel::update(audits::table.filter(audits::id.eq(audit_id)))
            .set(audits::end_date.eq(None::<chrono::NaiveDateTime>))
            .execute(&mut conn)?;
    } else if let Some(end_date) = &payload.end_date {
        diesel::update(audits::table.filter(audits::id.eq(audit_id)))
            .set(audits::end_date.eq(end_date))
            .execute(&mut conn)?;
    }

    let (updated_audit, company) = audits::table
        .filter(audits::id.eq(audit_id))
        .inner_join(companies::table.on(audits::company_id.eq(companies::id)))
        .select((Audit::as_select(), Company::as_select()))
        .first::<(Audit, Company)>(&mut conn)?;

    let client_users = audits_clients::table
        .filter(audits_clients::audit_id.eq(audit_id))
        .inner_join(users::table.on(audits_clients::user_id.eq(users::id)))
        .select(User::as_select())
        .load::<User>(&mut conn)?;

    Ok(AuditResponse::from_audit_and_clients(
        updated_audit,
        company.into(),
        client_users,
    ))
}

/// Archive an audit (soft delete)
///
/// # Errors
/// Returns `AuditError` if validation fails or database error
pub fn archive_audit(
    state: &GlobalState,
    audit_id: &str,
    user_id: &str,
    is_admin: bool,
) -> Result<(), AuditError> {
    use crate::schema::audits;

    let mut audit = get_audit(state, audit_id)?;
    auto_archive_if_needed(state, &mut audit)?;

    if audit.is_archived {
        return Err(AuditError::AuditArchived);
    }

    if !can_modify_audit(state, &audit, user_id, is_admin)? {
        return Err(AuditError::MustBeOwnerOrAdmin);
    }

    let mut conn = state.get_db_conn()?;
    diesel::update(audits::table.filter(audits::id.eq(audit_id)))
        .set(audits::is_archived.eq(true))
        .execute(&mut conn)?;

    Ok(())
}

/// Remove a client from audit
///
/// # Errors
/// Returns `AuditError` if validation fails or database error
pub fn remove_client_from_audit(
    state: &GlobalState,
    audit_id: &str,
    client_user_id: &str,
    requester_id: &str,
    is_admin: bool,
) -> Result<(), AuditError> {
    use crate::schema::audits_clients;

    let mut audit = get_audit(state, audit_id)?;
    auto_archive_if_needed(state, &mut audit)?;

    if !is_admin {
        let user_teams = get_user_team_ids(state, requester_id)?;
        if !user_teams.contains(&audit.team_id) {
            return Err(AuditError::NotFound);
        }
    }

    if audit.is_archived {
        return Err(AuditError::AuditArchived);
    }

    if !can_modify_audit(state, &audit, requester_id, is_admin)? {
        return Err(AuditError::MustBeOwnerOrAdmin);
    }

    if !is_client_in_audit(state, audit_id, client_user_id)? {
        return Err(AuditError::ClientNotInAudit);
    }

    let mut conn = state.get_db_conn()?;
    diesel::delete(
        audits_clients::table
            .filter(audits_clients::audit_id.eq(audit_id))
            .filter(audits_clients::user_id.eq(client_user_id)),
    )
    .execute(&mut conn)?;

    Ok(())
}
