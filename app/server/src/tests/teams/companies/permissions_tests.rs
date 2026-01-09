use crate::api::teams::TeamRole;
use crate::tests::teams::companies::permissions::{can_manage_companies, can_read_companies};

#[test]
fn owner_can_manage_companies() {
    assert!(can_manage_companies(TeamRole::Owner, true));
}

#[test]
fn moderator_can_manage_companies() {
    assert!(can_manage_companies(TeamRole::Moderator, true));
}

#[test]
fn member_cannot_manage_companies() {
    assert!(!can_manage_companies(TeamRole::Member, true));
}

#[test]
fn client_cannot_manage_companies() {
    assert!(!can_manage_companies(TeamRole::Client, true));
}

#[test]
fn non_member_cannot_manage_companies() {
    assert!(!can_manage_companies(TeamRole::Owner, false));
    assert!(!can_manage_companies(TeamRole::Moderator, false));
    assert!(!can_manage_companies(TeamRole::Member, false));
    assert!(!can_manage_companies(TeamRole::Client, false));
}

#[test]
fn all_members_can_read_companies() {
    assert!(can_read_companies(true));
}

#[test]
fn non_members_cannot_read_companies() {
    assert!(!can_read_companies(false));
}
