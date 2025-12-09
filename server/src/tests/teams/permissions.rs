use crate::api::teams::TeamRole;

pub fn can_edit_team(actor_role: TeamRole, actor_is_member: bool) -> bool {
    actor_is_member && actor_role == TeamRole::Owner
}

pub fn can_create_invitation(actor_role: TeamRole, actor_is_member: bool) -> bool {
    actor_is_member && (actor_role == TeamRole::Owner || actor_role == TeamRole::Moderator)
}

pub fn can_remove_member(
    actor_role: TeamRole,
    target_role: TeamRole,
    actor_is_member: bool,
    target_is_member: bool,
    is_self: bool,
) -> bool {
    if !actor_is_member || !target_is_member {
        return false;
    }

    if is_self {
        return true;
    }

    actor_role == TeamRole::Owner
        || (actor_role == TeamRole::Moderator && target_role != TeamRole::Owner)
}

pub fn can_change_role(
    actor_role: TeamRole,
    actor_is_member: bool,
    target_is_member: bool,
) -> bool {
    actor_is_member && target_is_member && actor_role == TeamRole::Owner
}
