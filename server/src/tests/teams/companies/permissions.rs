use crate::api::teams::TeamRole;

/// Check if a user can create/update/delete companies
pub fn can_manage_companies(actor_role: TeamRole, actor_is_member: bool) -> bool {
    actor_is_member && (actor_role == TeamRole::Owner || actor_role == TeamRole::Moderator)
}

/// Check if a user can read companies
pub fn can_read_companies(actor_is_member: bool) -> bool {
    actor_is_member
}
