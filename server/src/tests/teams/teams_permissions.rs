use crate::api::teams::TeamRole;
use crate::tests::teams::permissions::{
    can_change_role, can_create_invitation, can_edit_team, can_remove_member,
};

#[test]
fn non_member_cannot_crud_team_or_members() {
    assert!(!can_edit_team(TeamRole::Owner, false));
    assert!(!can_create_invitation(TeamRole::Owner, false));
    assert!(!can_remove_member(
        TeamRole::Owner,
        TeamRole::Member,
        false,
        true,
        false
    ));
    assert!(!can_change_role(TeamRole::Owner, false, true));
}

#[test]
fn member_or_client_cannot_cud_without_admin_mod_role() {
    for role in [TeamRole::Member, TeamRole::Client] {
        assert!(!can_edit_team(role.clone(), true));
        assert!(!can_create_invitation(role.clone(), true));
        assert!(!can_remove_member(
            role.clone(),
            TeamRole::Member,
            true,
            true,
            false
        ));
        assert!(!can_change_role(role, true, true));
    }
}

#[test]
fn actions_fail_when_target_not_in_team() {
    assert!(!can_remove_member(
        TeamRole::Owner,
        TeamRole::Member,
        true,
        false,
        false
    ));
    assert!(!can_change_role(TeamRole::Owner, true, false));
}
