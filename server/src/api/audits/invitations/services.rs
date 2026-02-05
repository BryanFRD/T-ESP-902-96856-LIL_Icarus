use crate::{
    api::{
        audits::{AuditError, get_audit, models::AuditClient},
        auth::JwtClaims,
        teams::{TeamError, is_team_owner_or_moderator},
        users::{User, UserError, create_user, get_user_by_email},
    },
    mails::send_audit_invitation_mail,
    schema::{audits_clients, audits_invitations, users},
    state::GlobalState,
};
use chrono::{Duration, Utc};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};

use super::{
    AcceptAuditInvitationPayload, AcceptInvitationResponse, AuditInvitation, AuditInvitationError,
    AuditInvitationToken, CheckInvitationResponse,
};

pub async fn create_audit_invitation(
    state: &GlobalState,
    audit_id: &str,
    creator_user_id: &str,
    client_email: String,
    expires_at: Option<chrono::NaiveDateTime>,
    is_admin: bool,
) -> Result<AuditInvitation, AuditInvitationError> {
    validate_invitation_creation(state, audit_id, creator_user_id, &client_email, is_admin)?;

    let token = generate_invitation_token(audit_id, &client_email, state.get_secrets())?;

    let expires_at = expires_at.unwrap_or_else(|| (Utc::now() + Duration::hours(24)).naive_utc());

    let invitation = insert_invitation(
        state,
        audit_id,
        &client_email,
        token.clone(),
        expires_at,
        creator_user_id,
    )?;

    send_invitation_email(state, &invitation, audit_id, token).await?;

    Ok(invitation)
}

pub fn check_invitation(
    state: &GlobalState,
    token: &str,
) -> Result<CheckInvitationResponse, AuditInvitationError> {
    let token_data = decrypt_and_validate_token(token, state.get_secrets())?;
    let invitation = find_and_validate_invitation(state, token)?;
    let audit = get_audit(state, &token_data.audit_id)?;
    let user_exists = get_user_by_email(state, &token_data.client_email)?.is_some();

    Ok(CheckInvitationResponse {
        audit_name: audit.name,
        audit_id: audit.id,
        client_email: token_data.client_email,
        needs_account_creation: !user_exists,
        expires_at: invitation.expires_at.to_string(),
    })
}

pub fn accept_audit_invitation(
    state: &GlobalState,
    token: &str,
    payload: AcceptAuditInvitationPayload,
    authenticated_user: Option<&JwtClaims>,
) -> Result<AcceptInvitationResponse, AuditInvitationError> {
    let (token_data, invitation, audit) = validate_invitation_acceptance(state, token)?;

    let (user_id, is_new) =
        resolve_or_create_user(state, &token_data, payload, authenticated_user)?;

    add_user_to_audit(state, &audit.id, &user_id)?;
    mark_invitation_as_used(state, &invitation.id)?;

    build_response(state, user_id, audit.id, is_new)
}

pub fn list_audit_invitations(
    state: &GlobalState,
    audit_id: &str,
    requester_user_id: &str,
    is_admin: bool,
) -> Result<Vec<AuditInvitation>, AuditInvitationError> {
    let audit = get_audit(state, audit_id)?;

    if !is_admin && !is_team_owner_or_moderator(state, &audit.team_id, requester_user_id)? {
        return Err(AuditInvitationError::NotAuthorized);
    }

    let mut conn = state.get_db_conn()?;

    let invitations = audits_invitations::table
        .filter(audits_invitations::audit_id.eq(audit_id))
        .select(AuditInvitation::as_select())
        .load::<AuditInvitation>(&mut conn)?;

    Ok(invitations)
}

pub fn cancel_audit_invitation(
    state: &GlobalState,
    audit_id: &str,
    invitation_id: &str,
    requester_user_id: &str,
    is_admin: bool,
) -> Result<(), AuditInvitationError> {
    let audit = get_audit(state, audit_id)?;

    if !is_admin && !is_team_owner_or_moderator(state, &audit.team_id, requester_user_id)? {
        return Err(AuditInvitationError::NotAuthorized);
    }

    let mut conn = state.get_db_conn()?;

    let affected = diesel::update(
        audits_invitations::table
            .filter(audits_invitations::id.eq(invitation_id))
            .filter(audits_invitations::audit_id.eq(audit_id)),
    )
    .set(audits_invitations::is_canceled.eq(true))
    .execute(&mut conn)?;

    if affected == 0 {
        return Err(AuditInvitationError::NotFound);
    }

    Ok(())
}

fn validate_invitation_creation(
    state: &GlobalState,
    audit_id: &str,
    creator_user_id: &str,
    client_email: &str,
    is_admin: bool,
) -> Result<(), AuditInvitationError> {
    let audit = get_audit(state, audit_id)?;

    if audit.is_archived {
        return Err(AuditInvitationError::AuditError(AuditError::AuditArchived));
    }

    if !is_admin && !is_team_owner_or_moderator(state, &audit.team_id, creator_user_id)? {
        return Err(AuditInvitationError::NotAuthorized);
    }

    if is_email_already_in_audit(state, audit_id, client_email)? {
        return Err(AuditInvitationError::EmailAlreadyInAudit);
    }

    Ok(())
}

fn generate_invitation_token(
    audit_id: &str,
    client_email: &str,
    secrets: &crate::vault::Secrets,
) -> Result<String, AuditInvitationError> {
    let token_data = AuditInvitationToken::new(audit_id.to_string(), client_email.to_string());
    token_data.encrypt(secrets.jwt_secret.as_bytes())
}

fn insert_invitation(
    state: &GlobalState,
    audit_id: &str,
    client_email: &str,
    token: String,
    expires_at: chrono::NaiveDateTime,
    created_by: &str,
) -> Result<AuditInvitation, AuditInvitationError> {
    let mut conn = state.get_db_conn()?;

    let invitation = AuditInvitation::new(
        audit_id.to_string(),
        client_email.to_string(),
        token,
        expires_at,
        created_by.to_string(),
    );

    diesel::insert_into(audits_invitations::table)
        .values(&invitation)
        .execute(&mut conn)?;

    Ok(invitation)
}

async fn send_invitation_email(
    state: &GlobalState,
    invitation: &AuditInvitation,
    audit_id: &str,
    token: String,
) -> Result<(), AuditInvitationError> {
    let audit = get_audit(state, audit_id)?;

    let secrets = state.get_secrets();
    if secrets.resend_api_key == "test_resend_api_key" {
        tracing::info!("Test mode: Skipping invitation email send");
        return Ok(());
    }

    let resend = state.get_resend().lock().await;

    send_audit_invitation_mail(
        token,
        audit.name,
        audit_id.to_string(),
        invitation.client_email.clone(),
        &resend,
    )
    .await?;

    Ok(())
}

fn decrypt_and_validate_token(
    token: &str,
    secrets: &crate::vault::Secrets,
) -> Result<AuditInvitationToken, AuditInvitationError> {
    AuditInvitationToken::decrypt(token, secrets.jwt_secret.as_bytes())
}

fn find_and_validate_invitation(
    state: &GlobalState,
    token: &str,
) -> Result<AuditInvitation, AuditInvitationError> {
    let mut conn = state.get_db_conn()?;

    let invitation = audits_invitations::table
        .filter(audits_invitations::token.eq(token))
        .select(AuditInvitation::as_select())
        .first::<AuditInvitation>(&mut conn)
        .optional()?
        .ok_or(AuditInvitationError::NotFound)?;

    if invitation.is_canceled {
        return Err(AuditInvitationError::Canceled);
    }

    if invitation.is_used {
        return Err(AuditInvitationError::AlreadyUsed);
    }

    if invitation.is_expired() {
        return Err(AuditInvitationError::Expired);
    }

    Ok(invitation)
}

fn validate_invitation_acceptance(
    state: &GlobalState,
    token: &str,
) -> Result<
    (
        AuditInvitationToken,
        AuditInvitation,
        crate::api::audits::Audit,
    ),
    AuditInvitationError,
> {
    let token_data = decrypt_and_validate_token(token, state.get_secrets())?;
    let invitation = find_and_validate_invitation(state, token)?;
    let audit = get_audit(state, &token_data.audit_id)?;

    if audit.is_archived {
        return Err(AuditInvitationError::AuditError(AuditError::AuditArchived));
    }

    Ok((token_data, invitation, audit))
}

fn resolve_or_create_user(
    state: &GlobalState,
    token_data: &AuditInvitationToken,
    payload: AcceptAuditInvitationPayload,
    authenticated_user: Option<&JwtClaims>,
) -> Result<(String, bool), AuditInvitationError> {
    let existing_user = get_user_by_email(state, &token_data.client_email)?;

    match (existing_user, authenticated_user) {
        (Some(user), Some(jwt_user)) => {
            if jwt_user.email.to_lowercase() != token_data.client_email.to_lowercase() {
                return Err(AuditInvitationError::EmailMismatch);
            }
            Ok((user.id, false))
        }
        (Some(_), None) => Err(AuditInvitationError::AuthenticationRequired),
        (None, _) => {
            let user_id = create_client_account(state, token_data, payload)?;
            Ok((user_id, true))
        }
    }
}

fn create_client_account(
    state: &GlobalState,
    token_data: &AuditInvitationToken,
    payload: AcceptAuditInvitationPayload,
) -> Result<String, AuditInvitationError> {
    let username = payload
        .username
        .ok_or(AuditInvitationError::MissingAccountDetails)?;
    let first_name = payload
        .first_name
        .ok_or(AuditInvitationError::MissingAccountDetails)?;
    let last_name = payload
        .last_name
        .ok_or(AuditInvitationError::MissingAccountDetails)?;
    let password = payload
        .password
        .ok_or(AuditInvitationError::MissingAccountDetails)?;

    if password.len() < 8 {
        return Err(AuditInvitationError::PasswordTooShort);
    }

    if username_exists(state, &username)? {
        return Err(AuditInvitationError::UsernameAlreadyTaken);
    }

    let new_user = User::new(
        username,
        first_name,
        last_name,
        token_data.client_email.clone(),
        "client".to_string(),
        &password,
    );

    let user_id = new_user.id.clone();
    create_user(state, &new_user)?;

    Ok(user_id)
}

fn add_user_to_audit(
    state: &GlobalState,
    audit_id: &str,
    user_id: &str,
) -> Result<(), AuditInvitationError> {
    if is_user_in_audit(state, audit_id, user_id)? {
        return Err(AuditInvitationError::EmailAlreadyInAudit);
    }

    let mut conn = state.get_db_conn()?;
    let audit_client = AuditClient::new(audit_id.to_string(), user_id.to_string());

    diesel::insert_into(audits_clients::table)
        .values(&audit_client)
        .execute(&mut conn)?;

    Ok(())
}

fn mark_invitation_as_used(
    state: &GlobalState,
    invitation_id: &str,
) -> Result<(), AuditInvitationError> {
    let mut conn = state.get_db_conn()?;

    diesel::update(audits_invitations::table.filter(audits_invitations::id.eq(invitation_id)))
        .set((
            audits_invitations::is_used.eq(true),
            audits_invitations::used_at.eq(Some(Utc::now().naive_utc())),
        ))
        .execute(&mut conn)?;

    Ok(())
}

fn build_response(
    state: &GlobalState,
    user_id: String,
    audit_id: String,
    is_new_account: bool,
) -> Result<AcceptInvitationResponse, AuditInvitationError> {
    if is_new_account {
        let user = get_user_by_email(state, &get_user_email_by_id(state, &user_id)?)?
            .ok_or(AuditInvitationError::UserError(UserError::UserNotFound))?;

        let jwt = JwtClaims::from(user);
        let jwt_string = jwt
            .to_string(&state.get_secrets().jwt_secret)
            .map_err(|_| AuditInvitationError::JwtGenerationFailed)?;

        Ok(AcceptInvitationResponse::AccountCreated {
            user_id,
            audit_id,
            jwt: jwt_string,
        })
    } else {
        Ok(AcceptInvitationResponse::AccountExists { audit_id })
    }
}

fn username_exists(state: &GlobalState, username: &str) -> Result<bool, AuditInvitationError> {
    let mut conn = state.get_db_conn()?;
    let count = users::table
        .filter(users::username.eq(username))
        .count()
        .get_result::<i64>(&mut conn)?;

    Ok(count > 0)
}

fn is_email_already_in_audit(
    state: &GlobalState,
    audit_id: &str,
    email: &str,
) -> Result<bool, AuditInvitationError> {
    let user = get_user_by_email(state, email)?;

    if let Some(user) = user {
        return is_user_in_audit(state, audit_id, &user.id);
    }

    Ok(false)
}

fn is_user_in_audit(
    state: &GlobalState,
    audit_id: &str,
    user_id: &str,
) -> Result<bool, AuditInvitationError> {
    let mut conn = state.get_db_conn()?;
    let count = audits_clients::table
        .filter(audits_clients::audit_id.eq(audit_id))
        .filter(audits_clients::user_id.eq(user_id))
        .count()
        .get_result::<i64>(&mut conn)?;

    Ok(count > 0)
}

fn get_user_email_by_id(
    state: &GlobalState,
    user_id: &str,
) -> Result<String, AuditInvitationError> {
    let mut conn = state.get_db_conn()?;
    let email = users::table
        .filter(users::id.eq(user_id))
        .select(users::email)
        .first::<String>(&mut conn)
        .optional()?
        .ok_or(AuditInvitationError::UserError(UserError::UserNotFound))?;

    Ok(email)
}

impl From<TeamError> for AuditInvitationError {
    fn from(_: TeamError) -> Self {
        AuditInvitationError::NotAuthorized
    }
}
