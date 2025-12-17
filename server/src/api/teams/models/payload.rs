use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::api::{teams::{Team, CompanyReturn}, users::User};
use crate::api::teams::TeamRole;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateTeam {
    #[schema(example = "Example Team")]
    pub name: String,
    #[schema(example = "This is an example team description.")]
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateTeam {
    #[schema(example = "Example Team")]
    pub name: Option<String>,
    #[schema(example = "This is an example team description.")]
    pub description: Option<String>,
}

#[derive(Serialize, Debug, Clone, ToSchema)]
pub struct TeamReturn {
    #[schema(example = "xxxxxxxxxxxxxxxxxxxxxx")]
    pub id: String,
    #[schema(example = "Example Team")]
    pub name: String,
    #[schema(example = "This is an example team description.")]
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
    pub members: Vec<TeamMemberWithRole>,
    pub companies: Vec<CompanyReturn>,
}

impl TeamReturn {
    pub fn new(team: Team, members: Vec<TeamMemberWithRole>, companies: Vec<CompanyReturn>) -> Self {
        TeamReturn {
            id: team.id,
            name: team.name,
            description: team.description,
            created_at: team.created_at.to_string(),
            updated_at: team.updated_at.to_string(),
            members,
            companies,
        }
    }
}

#[derive(Serialize, Debug, Clone, ToSchema)]
pub struct TeamMemberWithRole {
    #[serde(rename = "team_role")]
    pub team_role: TeamRole,
    #[serde(rename = "user_role")]
    pub user_role: crate::api::users::UserRole,
    #[serde(flatten)]
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateMemberRolePayload {
    pub role: TeamRole,
}
