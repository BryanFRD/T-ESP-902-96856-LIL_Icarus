use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{teams::Team, users::User};

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct CreateTeam {
    #[schema(example = "Example Team")]
    pub name: String,
    #[schema(example = "This is an example team description.")]
    pub description: String,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct UpdateTeam {
    #[schema(example = "Example Team")]
    pub name: Option<String>,
    #[schema(example = "This is an example team description.")]
    pub description: Option<String>,
}

#[derive(Serialize, Debug, Clone, ToSchema)]
pub struct TeamReturn {
    #[schema(example = "01F8MECHZX3TBDSZ7XK4F5G9ZQ")]
    pub id: String,
    #[schema(example = "Example Team")]
    pub name: String,
    #[schema(example = "This is an example team description.")]
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
    pub members: Vec<User>,
}

impl TeamReturn {
    pub fn new(team: Team, members: Vec<User>) -> Self {
        TeamReturn {
            id: team.id,
            name: team.name,
            description: team.description,
            created_at: team.created_at.to_string(),
            updated_at: team.updated_at.to_string(),
            members,
        }
    }
}
