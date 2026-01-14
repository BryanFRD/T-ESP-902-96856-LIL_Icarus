use crate::api::events::logging::{
    RoutePattern, infer_delete_event, infer_get_event, infer_patch_event, infer_post_event,
};
use crate::api::events::{EventAction, ResourceType};

#[test]
fn test_route_pattern_parsing() {
    let route = RoutePattern::new("GET", "/v1/users/me");
    assert_eq!(route.method, "GET");
    assert_eq!(route.get(0), Some("users"));
    assert_eq!(route.get(1), Some("me"));
    assert_eq!(route.get(2), None);
}

#[test]
fn test_route_pattern_with_multiple_segments() {
    let route = RoutePattern::new("POST", "/v1/teams/123/invitations");
    assert_eq!(route.get(0), Some("teams"));
    assert_eq!(route.get(1), Some("123"));
    assert_eq!(route.get(2), Some("invitations"));
}

#[test]
fn test_infer_post_login() {
    let route = RoutePattern::new("POST", "/v1/auth/login");
    let result = infer_post_event(&route);

    assert!(result.is_some());
    let (action, resource_type, resource_id) = result.unwrap();
    assert_eq!(action, EventAction::Login);
    assert_eq!(resource_type, ResourceType::Auth);
    assert_eq!(resource_id, None);
}

#[test]
fn test_infer_post_register() {
    let route = RoutePattern::new("POST", "/v1/auth/register");
    let result = infer_post_event(&route);

    assert!(result.is_some());
    let (action, resource_type, _) = result.unwrap();
    assert_eq!(action, EventAction::Register);
    assert_eq!(resource_type, ResourceType::Auth);
}

#[test]
fn test_infer_post_logout() {
    let route = RoutePattern::new("POST", "/v1/auth/logout");
    let result = infer_post_event(&route);

    assert!(result.is_some());
    let (action, resource_type, _) = result.unwrap();
    assert_eq!(action, EventAction::Logout);
    assert_eq!(resource_type, ResourceType::Auth);
}

#[test]
fn test_infer_post_create_team() {
    let route = RoutePattern::new("POST", "/v1/teams/me");
    let result = infer_post_event(&route);

    assert!(result.is_some());
    let (action, resource_type, resource_id) = result.unwrap();
    assert_eq!(action, EventAction::Create);
    assert_eq!(resource_type, ResourceType::Team);
    assert_eq!(resource_id, None);
}

#[test]
fn test_infer_post_create_company() {
    let route = RoutePattern::new("POST", "/v1/teams/abc123/company");
    let result = infer_post_event(&route);

    assert!(result.is_some());
    let (action, resource_type, resource_id) = result.unwrap();
    assert_eq!(action, EventAction::Create);
    assert_eq!(resource_type, ResourceType::Company);
    assert_eq!(resource_id, Some("team_id:abc123".to_string()));
}

#[test]
fn test_infer_post_create_invitation() {
    let route = RoutePattern::new("POST", "/v1/teams/xyz789/invitations");
    let result = infer_post_event(&route);

    assert!(result.is_some());
    let (action, resource_type, resource_id) = result.unwrap();
    assert_eq!(action, EventAction::InvitationCreate);
    assert_eq!(resource_type, ResourceType::Invitation);
    assert_eq!(resource_id, Some("team_id:xyz789".to_string()));
}

#[test]
fn test_infer_post_avatar_upload() {
    let route = RoutePattern::new("POST", "/v1/users/me/avatar");
    let result = infer_post_event(&route);

    assert!(result.is_some());
    let (action, resource_type, _) = result.unwrap();
    assert_eq!(action, EventAction::AvatarUpload);
    assert_eq!(resource_type, ResourceType::Avatar);
}

#[test]
fn test_infer_get_user_me() {
    let route = RoutePattern::new("GET", "/v1/users/me");
    let result = infer_get_event(&route);

    assert!(result.is_some());
    let (action, resource_type, resource_id) = result.unwrap();
    assert_eq!(action, EventAction::Read);
    assert_eq!(resource_type, ResourceType::User);
    assert_eq!(resource_id, None);
}

#[test]
fn test_infer_get_user_by_id() {
    let route = RoutePattern::new("GET", "/v1/users/user123");
    let result = infer_get_event(&route);

    assert!(result.is_some());
    let (action, resource_type, resource_id) = result.unwrap();
    assert_eq!(action, EventAction::Read);
    assert_eq!(resource_type, ResourceType::User);
    assert_eq!(resource_id, Some("user_id:user123".to_string()));
}

#[test]
fn test_infer_get_team_me() {
    let route = RoutePattern::new("GET", "/v1/teams/me");
    let result = infer_get_event(&route);

    assert!(result.is_some());
    let (action, resource_type, _) = result.unwrap();
    assert_eq!(action, EventAction::Read);
    assert_eq!(resource_type, ResourceType::Team);
}

#[test]
fn test_infer_get_team_by_id() {
    let route = RoutePattern::new("GET", "/v1/teams/team456");
    let result = infer_get_event(&route);

    assert!(result.is_some());
    let (action, resource_type, resource_id) = result.unwrap();
    assert_eq!(action, EventAction::Read);
    assert_eq!(resource_type, ResourceType::Team);
    assert_eq!(resource_id, Some("team_id:team456".to_string()));
}

#[test]
fn test_infer_get_company() {
    let route = RoutePattern::new("GET", "/v1/teams/team789/company");
    let result = infer_get_event(&route);

    assert!(result.is_some());
    let (action, resource_type, resource_id) = result.unwrap();
    assert_eq!(action, EventAction::Read);
    assert_eq!(resource_type, ResourceType::Company);
    assert_eq!(resource_id, Some("team_id:team789".to_string()));
}

#[test]
fn test_infer_patch_user_me() {
    let route = RoutePattern::new("PATCH", "/v1/users/me");
    let result = infer_patch_event(&route);

    assert!(result.is_some());
    let (action, resource_type, _) = result.unwrap();
    assert_eq!(action, EventAction::Update);
    assert_eq!(resource_type, ResourceType::User);
}

#[test]
fn test_infer_patch_user_by_id() {
    let route = RoutePattern::new("PATCH", "/v1/users/user999");
    let result = infer_patch_event(&route);

    assert!(result.is_some());
    let (action, resource_type, resource_id) = result.unwrap();
    assert_eq!(action, EventAction::Update);
    assert_eq!(resource_type, ResourceType::User);
    assert_eq!(resource_id, Some("user_id:user999".to_string()));
}

#[test]
fn test_infer_patch_team() {
    let route = RoutePattern::new("PATCH", "/v1/teams/teamabc");
    let result = infer_patch_event(&route);

    assert!(result.is_some());
    let (action, resource_type, resource_id) = result.unwrap();
    assert_eq!(action, EventAction::Update);
    assert_eq!(resource_type, ResourceType::Team);
    assert_eq!(resource_id, Some("team_id:teamabc".to_string()));
}

#[test]
fn test_infer_patch_company() {
    let route = RoutePattern::new("PATCH", "/v1/teams/teamxyz/company/comp123");
    let result = infer_patch_event(&route);

    assert!(result.is_some());
    let (action, resource_type, resource_id) = result.unwrap();
    assert_eq!(action, EventAction::Update);
    assert_eq!(resource_type, ResourceType::Company);
    assert_eq!(
        resource_id,
        Some("team_id:teamxyz,company_id:comp123".to_string())
    );
}

#[test]
fn test_infer_patch_team_member() {
    let route = RoutePattern::new("PATCH", "/v1/teams/team123/members/user456");
    let result = infer_patch_event(&route);

    assert!(result.is_some());
    let (action, resource_type, resource_id) = result.unwrap();
    assert_eq!(action, EventAction::Update);
    assert_eq!(resource_type, ResourceType::TeamMember);
    assert_eq!(resource_id, Some("team_id:team123,user_id:user456".to_string()));
}

#[test]
fn test_infer_delete_user_me() {
    let route = RoutePattern::new("DELETE", "/v1/users/me");
    let result = infer_delete_event(&route);

    assert!(result.is_some());
    let (action, resource_type, _) = result.unwrap();
    assert_eq!(action, EventAction::Delete);
    assert_eq!(resource_type, ResourceType::User);
}

#[test]
fn test_infer_delete_avatar() {
    let route = RoutePattern::new("DELETE", "/v1/users/me/avatar");
    let result = infer_delete_event(&route);

    assert!(result.is_some());
    let (action, resource_type, _) = result.unwrap();
    assert_eq!(action, EventAction::AvatarDelete);
    assert_eq!(resource_type, ResourceType::Avatar);
}

#[test]
fn test_infer_delete_team() {
    let route = RoutePattern::new("DELETE", "/v1/teams/teamdel");
    let result = infer_delete_event(&route);

    assert!(result.is_some());
    let (action, resource_type, resource_id) = result.unwrap();
    assert_eq!(action, EventAction::Delete);
    assert_eq!(resource_type, ResourceType::Team);
    assert_eq!(resource_id, Some("team_id:teamdel".to_string()));
}

#[test]
fn test_infer_delete_company() {
    let route = RoutePattern::new("DELETE", "/v1/teams/team888/company/comp999");
    let result = infer_delete_event(&route);

    assert!(result.is_some());
    let (action, resource_type, resource_id) = result.unwrap();
    assert_eq!(action, EventAction::Delete);
    assert_eq!(resource_type, ResourceType::Company);
    assert_eq!(
        resource_id,
        Some("team_id:team888,company_id:comp999".to_string())
    );
}

#[test]
fn test_infer_delete_team_member() {
    let route = RoutePattern::new("DELETE", "/v1/teams/team111/members/user222");
    let result = infer_delete_event(&route);

    assert!(result.is_some());
    let (action, resource_type, resource_id) = result.unwrap();
    assert_eq!(action, EventAction::Delete);
    assert_eq!(resource_type, ResourceType::TeamMember);
    assert_eq!(resource_id, Some("team_id:team111,user_id:user222".to_string()));
}

#[test]
fn test_infer_delete_invitation() {
    let route = RoutePattern::new("DELETE", "/v1/teams/team333/invitations/token444");
    let result = infer_delete_event(&route);

    assert!(result.is_some());
    let (action, resource_type, resource_id) = result.unwrap();
    assert_eq!(action, EventAction::InvitationCancel);
    assert_eq!(resource_type, ResourceType::Invitation);
    assert_eq!(resource_id, Some("team_id:team333,token:token444".to_string()));
}

#[test]
fn test_infer_unknown_route_returns_none() {
    let route = RoutePattern::new("GET", "/v1/unknown/endpoint");
    let result = infer_get_event(&route);
    assert!(result.is_none());
}

#[test]
fn test_infer_get_email_confirm() {
    let route = RoutePattern::new("GET", "/v1/auth/confirm-email");
    let result = infer_get_event(&route);

    assert!(result.is_some());
    let (action, resource_type, _) = result.unwrap();
    assert_eq!(action, EventAction::EmailConfirm);
    assert_eq!(resource_type, ResourceType::Auth);
}

#[test]
fn test_resource_id_format_with_multiple_ids() {
    let route = RoutePattern::new("PATCH", "/v1/teams/abc/members/def");
    let result = infer_patch_event(&route);

    assert!(result.is_some());
    let (_, _, resource_id) = result.unwrap();

    let id = resource_id.unwrap();
    assert!(id.contains("team_id:abc"));
    assert!(id.contains("user_id:def"));
    assert!(id.contains(","));
}
