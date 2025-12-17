use super::{Company, CompanyError, CreateCompanyPayload, UpdateCompanyPayload};
use crate::{
    api::teams::{does_team_exist, get_team_member, is_team_owner_or_moderator},
    state::GlobalState,
};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};

/// Creates a new company for a team
///
/// # Errors
///
/// Returns an error if:
/// - Database connection fails
/// - Team does not exist
/// - User is not authorized (not owner/moderator and not admin)
/// - SIRET already exists for this team
pub fn create_company(
    state: &GlobalState,
    team_id: &str,
    user_id: &str,
    payload: CreateCompanyPayload,
    is_admin: bool,
) -> Result<Company, CompanyError> {
    use crate::schema::companies;

    if !does_team_exist(state, team_id)? {
        return Err(CompanyError::TeamNotFound);
    }

    if !is_admin && !is_team_owner_or_moderator(state, team_id, user_id)? {
        return Err(CompanyError::NotAuthorized);
    }

    let new_company = Company::new(
        team_id.to_string(),
        payload.name,
        payload.description,
        payload.siret,
    );

    let mut conn = state.get_db_conn()?;
    diesel::insert_into(companies::table)
        .values(&new_company)
        .execute(&mut conn)
        .map_err(|e| {
            if let diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            ) = e
            {
                CompanyError::DuplicateSiret
            } else {
                CompanyError::Database(e)
            }
        })?;

    Ok(new_company)
}

/// Lists all companies for a team
///
/// # Errors
///
/// Returns an error if:
/// - Database connection fails
/// - Team does not exist
/// - User is not a team member
pub fn list_companies(
    state: &GlobalState,
    team_id: &str,
    user_id: &str,
) -> Result<Vec<Company>, CompanyError> {
    use crate::schema::companies;

    if !does_team_exist(state, team_id)? {
        return Err(CompanyError::TeamNotFound);
    }

    if get_team_member(state, team_id, user_id)?.is_none() {
        return Err(CompanyError::NotTeamMember);
    }

    let mut conn = state.get_db_conn()?;
    let companies = companies::table
        .filter(companies::team_id.eq(team_id))
        .select(Company::as_select())
        .load::<Company>(&mut conn)?;

    Ok(companies)
}

/// Gets a single company by ID
///
/// # Errors
///
/// Returns an error if:
/// - Database connection fails
/// - Team does not exist
/// - User is not a team member
/// - Company does not exist
/// - Company does not belong to the specified team
pub fn get_company(
    state: &GlobalState,
    team_id: &str,
    company_id: &str,
    user_id: &str,
) -> Result<Company, CompanyError> {
    use crate::schema::companies;

    if !does_team_exist(state, team_id)? {
        return Err(CompanyError::TeamNotFound);
    }

    if get_team_member(state, team_id, user_id)?.is_none() {
        return Err(CompanyError::NotTeamMember);
    }

    let mut conn = state.get_db_conn()?;
    let company = companies::table
        .filter(companies::id.eq(company_id))
        .select(Company::as_select())
        .first::<Company>(&mut conn)
        .optional()?
        .ok_or(CompanyError::NotFound)?;

    if company.team_id != team_id {
        return Err(CompanyError::NotFound);
    }

    Ok(company)
}

/// Updates a company
///
/// # Errors
///
/// Returns an error if:
/// - Database connection fails
/// - Team does not exist
/// - User is not authorized (not owner/moderator and not admin)
/// - Company does not exist
/// - Company does not belong to the specified team
/// - SIRET already exists for this team (if updating SIRET)
pub fn update_company(
    state: &GlobalState,
    team_id: &str,
    company_id: &str,
    user_id: &str,
    payload: UpdateCompanyPayload,
    is_admin: bool,
) -> Result<Company, CompanyError> {
    use crate::schema::companies;

    if !does_team_exist(state, team_id)? {
        return Err(CompanyError::TeamNotFound);
    }

    if !is_admin && !is_team_owner_or_moderator(state, team_id, user_id)? {
        return Err(CompanyError::NotAuthorized);
    }

    let mut conn = state.get_db_conn()?;
    let company = companies::table
        .filter(companies::id.eq(company_id))
        .select(Company::as_select())
        .first::<Company>(&mut conn)
        .optional()?
        .ok_or(CompanyError::NotFound)?;

    if company.team_id != team_id {
        return Err(CompanyError::NotFound);
    }

    let updated_company = diesel::update(companies::table.filter(companies::id.eq(company_id)))
        .set((
            payload.name.map(|n| companies::name.eq(n)),
            payload.siret.map(|s| companies::siret.eq(s)),
            payload.description.map(|d| companies::description.eq(d)),
        ))
        .get_result::<Company>(&mut conn)
        .map_err(|e| {
            if let diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            ) = e
            {
                CompanyError::DuplicateSiret
            } else {
                CompanyError::Database(e)
            }
        })?;

    Ok(updated_company)
}

/// Deletes a company
///
/// # Errors
///
/// Returns an error if:
/// - Database connection fails
/// - Team does not exist
/// - User is not authorized (not owner/moderator and not admin)
/// - Company does not exist
/// - Company does not belong to the specified team
pub fn delete_company(
    state: &GlobalState,
    team_id: &str,
    company_id: &str,
    user_id: &str,
    is_admin: bool,
) -> Result<(), CompanyError> {
    use crate::schema::companies;

    if !does_team_exist(state, team_id)? {
        return Err(CompanyError::TeamNotFound);
    }

    if !is_admin && !is_team_owner_or_moderator(state, team_id, user_id)? {
        return Err(CompanyError::NotAuthorized);
    }

    let mut conn = state.get_db_conn()?;
    let company = companies::table
        .filter(companies::id.eq(company_id))
        .select(Company::as_select())
        .first::<Company>(&mut conn)
        .optional()?
        .ok_or(CompanyError::NotFound)?;

    if company.team_id != team_id {
        return Err(CompanyError::NotFound);
    }

    diesel::delete(companies::table.filter(companies::id.eq(company_id))).execute(&mut conn)?;

    Ok(())
}
