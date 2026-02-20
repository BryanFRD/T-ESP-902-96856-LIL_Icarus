use crate::api::events::resource_fetcher::parse_resource_id;

#[test]
fn test_parse_resource_id_with_single_key() {
    let resource_id = "team_id:123";
    let result = parse_resource_id(resource_id, "team_id");
    assert_eq!(result, "123");
}

#[test]
fn test_parse_resource_id_with_multiple_keys() {
    let resource_id = "team_id:123,user_id:456";

    let team_id = parse_resource_id(resource_id, "team_id");
    assert_eq!(team_id, "123");

    let user_id = parse_resource_id(resource_id, "user_id");
    assert_eq!(user_id, "456");
}

#[test]
fn test_parse_resource_id_with_three_keys() {
    let resource_id = "team_id:123,user_id:456,company_id:789";

    assert_eq!(parse_resource_id(resource_id, "team_id"), "123");
    assert_eq!(parse_resource_id(resource_id, "user_id"), "456");
    assert_eq!(parse_resource_id(resource_id, "company_id"), "789");
}

#[test]
fn test_parse_resource_id_key_not_found() {
    let resource_id = "team_id:123";
    let result = parse_resource_id(resource_id, "user_id");
    // Should return the entire resource_id when key is not found
    assert_eq!(result, resource_id);
}

#[test]
fn test_parse_resource_id_empty_value() {
    let resource_id = "team_id:,user_id:456";
    let result = parse_resource_id(resource_id, "team_id");
    // Should extract empty string between colon and comma
    assert_eq!(result, "");
}

#[test]
fn test_parse_resource_id_with_uuid_values() {
    let resource_id =
        "team_id:550e8400-e29b-41d4-a716-446655440000,user_id:6ba7b810-9dad-11d1-80b4-00c04fd430c8";

    let team_id = parse_resource_id(resource_id, "team_id");
    assert_eq!(team_id, "550e8400-e29b-41d4-a716-446655440000");

    let user_id = parse_resource_id(resource_id, "user_id");
    assert_eq!(user_id, "6ba7b810-9dad-11d1-80b4-00c04fd430c8");
}

#[test]
fn test_parse_resource_id_key_in_middle() {
    let resource_id = "team_id:123,user_id:456,company_id:789";
    let result = parse_resource_id(resource_id, "user_id");
    assert_eq!(result, "456");
}

#[test]
fn test_parse_resource_id_key_at_end() {
    let resource_id = "team_id:123,user_id:456";
    let result = parse_resource_id(resource_id, "user_id");
    // Value at the end should go until end of string (no comma after it)
    assert_eq!(result, "456");
}

#[test]
fn test_parse_resource_id_with_numeric_id() {
    let resource_id = "team_id:12345";
    let result = parse_resource_id(resource_id, "team_id");
    assert_eq!(result, "12345");
}

#[test]
fn test_parse_resource_id_with_alphanumeric_id() {
    let resource_id = "team_id:abc123def456";
    let result = parse_resource_id(resource_id, "team_id");
    assert_eq!(result, "abc123def456");
}

#[test]
fn test_parse_resource_id_case_sensitive() {
    let resource_id = "Team_Id:123";
    let result = parse_resource_id(resource_id, "team_id");
    // Should return original since key case doesn't match
    assert_eq!(result, resource_id);
}

#[test]
fn test_parse_resource_id_similar_keys() {
    let resource_id = "team_id:123,team_id_sub:456";
    let result = parse_resource_id(resource_id, "team_id");
    // Should match the exact key "team_id:" and extract "123"
    assert_eq!(result, "123");
}

#[test]
fn test_parse_resource_id_with_special_characters_in_value() {
    let resource_id = "team_id:abc-def_123,user_id:xyz";
    let team_id = parse_resource_id(resource_id, "team_id");
    assert_eq!(team_id, "abc-def_123");
}

#[test]
fn test_parse_resource_id_empty_resource_id() {
    let resource_id = "";
    let result = parse_resource_id(resource_id, "team_id");
    // Should return empty string when key not found
    assert_eq!(result, "");
}

#[test]
fn test_parse_resource_id_only_key_no_value() {
    let resource_id = "team_id:";
    let result = parse_resource_id(resource_id, "team_id");
    // Should extract empty value when colon is present but no value follows
    assert_eq!(result, "");
}

#[test]
fn test_parse_resource_id_long_value() {
    let long_id = "a".repeat(1000);
    let resource_id = format!("team_id:{long_id}");
    let result = parse_resource_id(&resource_id, "team_id");
    assert_eq!(result, long_id);
}

#[test]
fn test_parse_resource_id_multiple_commas() {
    let resource_id = "team_id:123,user_id:456,company_id:789,role:admin";
    let team_id = parse_resource_id(resource_id, "team_id");
    assert_eq!(team_id, "123");

    let role = parse_resource_id(resource_id, "role");
    assert_eq!(role, "admin");
}
