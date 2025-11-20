use crate::api::teams::{does_team_exist, is_team_owner_or_moderator, TeamMember, TeamRole};
use crate::{
    schema::teams_invitations,
    state::GlobalState,
};

use super::{InvitationError, TeamInvitation};

pub fn create_invitation(
    state: &GlobalState,
    team_id: &str,
    creator_user_id: &str,
    max_uses: i32,
    expires_at: Option<chrono::NaiveDateTime>,
    is_admin: bool,
) -> Result<TeamInvitation, InvitationError> {
    use diesel::prelude::*;

    if !does_team_exist(state, team_id)? {
        return Err(InvitationError::TeamNotFound);
    }

    if !is_admin && !is_team_owner_or_moderator(state, team_id, creator_user_id)? {
        return Err(InvitationError::NotAuthorized);
    }

    let mut conn = state.get_db_conn()?;

    let invitation = TeamInvitation::new(
        team_id.to_string(),
        creator_user_id.to_string(),
        max_uses,
        expires_at,
    );

    diesel::insert_into(teams_invitations::table)
        .values(&invitation)
        .execute(&mut conn)?;

    Ok(invitation)
}

pub fn list_invitations(
    state: &GlobalState,
    team_id: &str,
    requester_user_id: &str,
    is_admin: bool,
) -> Result<Vec<TeamInvitation>, InvitationError> {
    use crate::schema::teams_invitations::dsl as inv;
    use diesel::prelude::*;

    if !does_team_exist(state, team_id)? {
        return Err(InvitationError::TeamNotFound);
    }

    if !is_admin && !is_team_owner_or_moderator(state, team_id, requester_user_id)? {
        return Err(InvitationError::NotAuthorized);
    }

    let mut conn = state.get_db_conn()?;

    let items = inv::teams_invitations
        .filter(inv::team_id.eq(team_id))
        .select(TeamInvitation::as_select())
        .load::<TeamInvitation>(&mut conn)?;

    Ok(items)
}

pub fn cancel_invitation(
    state: &GlobalState,
    team_id_str: &str,
    invitation_id: &str,
    requester_user_id: &str,
    is_admin: bool,
) -> Result<(), InvitationError> {
    use crate::schema::teams_invitations::dsl::{teams_invitations, id, team_id, is_canceled};
    use diesel::prelude::*;

    if !does_team_exist(state, team_id_str)? {
        return Err(InvitationError::TeamNotFound);
    }

    if !is_admin && !is_team_owner_or_moderator(state, team_id_str, requester_user_id)? {
        return Err(InvitationError::NotAuthorized);
    }

    let mut conn = state.get_db_conn()?;

    let affected = diesel::update(
        teams_invitations
            .filter(id.eq(invitation_id))
            .filter(team_id.eq(team_id_str)),
    )
    .set(is_canceled.eq(true))
    .execute(&mut conn)?;

    if affected == 0 {
        return Err(InvitationError::NotFound);
    }

    Ok(())
}

pub fn consume_invitation_by_token(
    state: &GlobalState,
    token_str: &str,
    joiner_user_id: &str,
) -> Result<String, InvitationError> {
    use crate::schema::{teams_invitations::dsl as inv, teams_members};
    use diesel::prelude::*;

    let mut conn = state.get_db_conn()?;

    let invitation = inv::teams_invitations
        .filter(inv::token.eq(token_str))
        .select(super::TeamInvitation::as_select())
        .first::<super::TeamInvitation>(&mut conn)
        .optional()?;

    let Some(invitation) = invitation else { return Err(InvitationError::NotFound) };

    if invitation.is_canceled {
        return Err(InvitationError::Canceled);
    }

    if let Some(exp) = invitation.expires_at {
        if chrono::Utc::now().naive_utc() > exp {
            return Err(InvitationError::Expired);
        }
    }

    if invitation.used_count >= invitation.max_uses {
        return Err(InvitationError::Exhausted);
    }

    let is_already_member: i64 = teams_members::table
        .filter(teams_members::team_id.eq(&invitation.team_id))
        .filter(teams_members::user_id.eq(joiner_user_id))
        .count()
        .get_result(&mut conn)?;

    if is_already_member > 0 {
        return Ok(invitation.team_id);
    }

    diesel::insert_into(teams_members::table)
        .values(&TeamMember {
            team_id: invitation.team_id.clone(),
            user_id: joiner_user_id.to_string(),
            role: TeamRole::Member,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        })
        .execute(&mut conn)?;

    diesel::update(inv::teams_invitations.filter(inv::id.eq(&invitation.id)))
        .set(inv::used_count.eq(invitation.used_count + 1))
        .execute(&mut conn)?;

    Ok(invitation.team_id)
}
