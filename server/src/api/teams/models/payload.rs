use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::api::teams::TeamRole;
use crate::api::{
    teams::{CompanyReturn, Team},
    users::User,
};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTeam {
    #[schema(example = "Example Team")]
    pub name: String,
    #[schema(example = "This is an example team description.")]
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTeam {
    #[schema(example = "Example Team")]
    pub name: Option<String>,
    #[schema(example = "This is an example team description.")]
    pub description: Option<String>,
}

#[derive(Serialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
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
    pub fn new(
        team: Team,
        members: Vec<TeamMemberWithRole>,
        companies: Vec<CompanyReturn>,
    ) -> Self {
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
#[serde(rename_all = "camelCase")]
pub struct TeamMemberWithRole {
    pub team_role: TeamRole,
    #[serde(flatten)]
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMemberRolePayload {
    pub role: TeamRole,
}
