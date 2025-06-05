use axum::{
    Router,
    middleware::from_fn,
    routing::{delete, get, patch, post},
};
use diesel::{
    BelongingToDsl, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, dsl::update,
};

use super::endpoints;
use crate::{
    state::GlobalState,
    teams::{Team, TeamMember, TeamReturn, TeamRole, UpdateTeam},
    users::{User, get_user_by_id},
};

pub fn get_routes(state: GlobalState) -> axum::Router {
    Router::new()
        .route("/{id}", delete(endpoints::delete_team))
        .route("/{id}", patch(endpoints::update_team))
        .route("/me", post(endpoints::create_team))
        .layer(from_fn(crate::auth::pentester_middleware))
        .route("/me", get(endpoints::get_my_teams))
        .with_state(state.clone())
}

pub fn create_team(
    state: &GlobalState,
    user_id: &str,
    name: String,
    description: String,
) -> Result<Team, diesel::result::Error> {
    use crate::schema::teams;
    let mut conn = match state.get_db_conn() {
        Ok(conn) => conn,
        Err(_) => return Err(diesel::result::Error::NotFound),
    };

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
) -> Result<(), diesel::result::Error> {
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
) -> Result<Vec<TeamReturn>, diesel::result::Error> {
    use crate::schema::{teams, teams_members, users};

    let user = match get_user_by_id(&state, user_id)? {
        Some(user) => user,
        None => return Err(diesel::result::Error::NotFound),
    };

    let mut conn = match state.get_db_conn() {
        Ok(conn) => conn,
        Err(_) => return Err(diesel::result::Error::NotFound),
    };

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

    use std::collections::HashMap;
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

pub fn does_team_exist(state: &GlobalState, team_id: &str) -> Result<bool, diesel::result::Error> {
    use crate::schema::teams::dsl::*;

    let mut conn = match state.get_db_conn() {
        Ok(conn) => conn,
        Err(_) => return Err(diesel::result::Error::NotFound),
    };

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
) -> Result<bool, diesel::result::Error> {
    use crate::schema::teams_members::dsl::*;

    let mut conn = match state.get_db_conn() {
        Ok(conn) => conn,
        Err(_) => return Err(diesel::result::Error::NotFound),
    };

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
) -> Result<usize, diesel::result::Error> {
    use crate::schema::teams::dsl::*;

    let mut conn = match state.get_db_conn() {
        Ok(conn) => conn,
        Err(_) => return Err(diesel::result::Error::NotFound),
    };

    let changes = update(teams.filter(id.eq(team_id)));

    let update_fields = (
        payload.name.as_ref().map(|n| name.eq(n)),
        payload.description.as_ref().map(|d| description.eq(d)),
    );

    changes
        .set(update_fields)
        .filter(id.eq(team_id))
        .execute(&mut conn)
}

pub fn delete_team(state: &GlobalState, team_id: &str) -> Result<usize, diesel::result::Error> {
    use crate::schema::teams::dsl::*;

    let mut conn = match state.get_db_conn() {
        Ok(conn) => conn,
        Err(_) => return Err(diesel::result::Error::NotFound),
    };

    diesel::delete(teams.filter(id.eq(team_id))).execute(&mut conn)
}
