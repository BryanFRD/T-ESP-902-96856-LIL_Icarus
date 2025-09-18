use axum::{
    Router,
    middleware::from_fn,
    routing::{delete, get, patch, post},
};
use diesel::{
    BelongingToDsl, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, dsl::update,
};
use std::collections::HashMap;
use super::endpoints;
use crate::{
    auth, state::GlobalState, teams::{Team, TeamError, TeamMember, TeamReturn, TeamRole, UpdateTeam}, users::{get_user_by_id, User, UserError}
};

pub fn get_routes(state: GlobalState) -> axum::Router {
    Router::new()
        .route("/{id}", delete(endpoints::delete_team))
        .route("/{id}", patch(endpoints::update_team))
        .route("/me", post(endpoints::create_team))
        .layer(from_fn(crate::auth::pentester_middleware))
        .route("/me", get(endpoints::get_my_teams))
        .layer(from_fn(auth::middleware))
        .with_state(state.clone())
}

pub fn create_team(
    state: &GlobalState,
    user_id: &str,
    name: String,
    description: String,
) -> Result<Team, TeamError> {
    use crate::schema::teams;
    let mut conn = state.get_db_conn()?;

    let new_team = Team::new(name, description);

    diesel::insert_into(teams::table)
        .values(&new_team)
        .execute(&mut conn)?;

    add_team_member(&mut conn, &new_team.id, user_id, TeamRole::Owner)?;

    Ok(new_team)
}

pub fn add_team_member(
    conn: &mut diesel::PgConnection,
    team_id: &str,
    user_id: &str,
    role: TeamRole,
) -> Result<(), TeamError> {
    use crate::schema::teams_members;

    let new_member = super::models::TeamMember {
        team_id: team_id.to_string(),
        user_id: user_id.to_string(),
        role,
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    diesel::insert_into(teams_members::table)
        .values(&new_member)
        .execute(conn)?;

    Ok(())
}

pub fn get_user_teams(
    state: &GlobalState,
    user_id: &str,
) -> Result<Vec<TeamReturn>, TeamError> {
    use crate::schema::{teams, teams_members, users};

    let user = match get_user_by_id(state, user_id) {
        Ok(Some(user)) => user,
        Ok(None) => return Err(TeamError::UserError(UserError::UserNotFound)),
        Err(e) => return Err(TeamError::UserError(e))
    };

    let mut conn = state.get_db_conn()?;

    let teams_vec = TeamMember::belonging_to(&user)
        .inner_join(teams::table)
        .select(Team::as_select())
        .load::<Team>(&mut conn)?;

    let team_ids: Vec<String> = teams_vec.iter().map(|t| t.id.clone()).collect();

    let team_members: Vec<(TeamMember, User)> = teams_members::table
        .filter(teams_members::team_id.eq_any(&team_ids))
        .inner_join(users::table)
        .select((TeamMember::as_select(), User::as_select()))
        .load::<(TeamMember, User)>(&mut conn)?;

    let mut members_map: HashMap<String, Vec<User>> = HashMap::new();
    for (tm, user) in team_members {
        members_map.entry(tm.team_id).or_default().push(user);
    }

    let result = teams_vec
        .into_iter()
        .map(|team| {
            let members = members_map.remove(&team.id).unwrap_or_default();
            TeamReturn::new(team, members)
        })
        .collect();

    Ok(result)
}

pub fn does_team_exist(state: &GlobalState, team_id: &str) -> Result<bool, TeamError> {
    use crate::schema::teams::dsl::*;

    let mut conn = state.get_db_conn()?;

    let count = teams
        .filter(id.eq(team_id))
        .count()
        .get_result::<i64>(&mut conn)?;

    Ok(count > 0)
}

pub fn is_team_owner(
    state: &GlobalState,
    team: &str,
    user: &str,
) -> Result<bool, TeamError> {
    use crate::schema::teams_members::dsl::*;

    let mut conn = state.get_db_conn()?;

    let count = teams_members
        .filter(team_id.eq(team))
        .filter(user_id.eq(user))
        .filter(role.eq("owner"))
        .count()
        .get_result::<i64>(&mut conn)?;

    Ok(count > 0)
}

pub fn update_team(
    state: &GlobalState,
    team_id: &str,
    payload: UpdateTeam,
) -> Result<usize, TeamError> {
    use crate::schema::teams::dsl::*;

    let mut conn = state.get_db_conn()?;

    let changes = update(teams.filter(id.eq(team_id)));

    let update_fields = (
        payload.name.as_ref().map(|n| name.eq(n)),
        payload.description.as_ref().map(|d| description.eq(d)),
    );

    let updated_rows = changes
        .set(update_fields)
        .filter(id.eq(team_id))
        .execute(&mut conn);

    match updated_rows {
        Ok(nb_changes) => Ok(nb_changes),
        Err(e) => Err(TeamError::DatabaseError(e))
    }
}

pub fn delete_team(state: &GlobalState, team_id: &str) -> Result<usize, TeamError> {
    use crate::schema::teams::dsl::*;

    let mut conn = state.get_db_conn()?;

    let deleted_rows = diesel::delete(teams.filter(id.eq(team_id))).execute(&mut conn);

    match deleted_rows {
        Ok(nb_changes) => Ok(nb_changes),
        Err(e) => Err(TeamError::DatabaseError(e))
    }
}
