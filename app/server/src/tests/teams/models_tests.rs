use crate::api::teams::Team;

// Tests for Team::new()
#[test]
fn test_team_new_creates_valid_team() {
    let team = Team::new(
        "Engineering Team".to_string(),
        "A team for software engineers".to_string(),
    );

    assert_eq!(team.name, "Engineering Team");
    assert_eq!(team.description, "A team for software engineers");
    assert!(!team.id.is_empty(), "Team ID should be generated");
}

#[test]
fn test_team_new_generates_ulid() {
    let team = Team::new("Test Team".to_string(), "Description".to_string());

    // ULID should be 26 characters
    assert_eq!(team.id.len(), 26, "Team ID should be a valid ULID (26 characters)");
}

#[test]
fn test_team_new_sets_timestamps() {
    let before = chrono::Utc::now().naive_utc();
    let team = Team::new("Team".to_string(), "Desc".to_string());
    let after = chrono::Utc::now().naive_utc();

    assert!(
        team.created_at >= before && team.created_at <= after,
        "created_at should be set to current time"
    );
    assert!(
        team.updated_at >= before && team.updated_at <= after,
        "updated_at should be set to current time"
    );
    assert_eq!(
        team.created_at, team.updated_at,
        "created_at and updated_at should be the same for new teams"
    );
}

#[test]
fn test_team_new_generates_unique_ids() {
    let team1 = Team::new("Team 1".to_string(), "First team".to_string());
    let team2 = Team::new("Team 2".to_string(), "Second team".to_string());

    assert_ne!(team1.id, team2.id, "Each team should have a unique ID");
}

#[test]
fn test_team_new_with_empty_name() {
    let team = Team::new("".to_string(), "Description".to_string());

    assert_eq!(team.name, "");
    assert!(!team.id.is_empty(), "Team should still be created with valid ID");
}

#[test]
fn test_team_new_with_empty_description() {
    let team = Team::new("Team Name".to_string(), "".to_string());

    assert_eq!(team.description, "");
    assert!(!team.id.is_empty(), "Team should still be created with valid ID");
}

#[test]
fn test_team_new_with_long_name() {
    let long_name = "A".repeat(1000);
    let team = Team::new(long_name.clone(), "Description".to_string());

    assert_eq!(team.name, long_name);
    assert_eq!(team.name.len(), 1000);
}

#[test]
fn test_team_new_with_special_characters() {
    let team = Team::new(
        "Team #1 @HQ 🚀".to_string(),
        "Description with émojis and spëcial çharacters!".to_string(),
    );

    assert_eq!(team.name, "Team #1 @HQ 🚀");
    assert_eq!(team.description, "Description with émojis and spëcial çharacters!");
}
