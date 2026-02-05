use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    diesel::Queryable,
    diesel::Identifiable,
    diesel::Insertable,
    diesel::Selectable,
    PartialEq,
    ToSchema,
)]
#[diesel(table_name = crate::schema::companies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct Company {
    pub id: String,
    pub team_id: String,
    pub name: String,
    pub description: Option<String>,
    pub siret: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Company {
    #[must_use]
    pub fn new(team_id: String, name: String, description: Option<String>, siret: String) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: ulid::Ulid::new().to_string(),
            team_id,
            name,
            description,
            siret,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateCompanyPayload {
    #[schema(example = "ACME Corp")]
    pub name: String,
    #[schema(example = "Leading provider of innovative solutions")]
    pub description: Option<String>,
    #[schema(example = "12345678901234")]
    pub siret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCompanyPayload {
    pub name: Option<String>,
    pub description: Option<String>,
    pub siret: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CompanyReturn {
    pub id: String,
    pub team_id: String,
    pub name: String,
    pub description: Option<String>,
    pub siret: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Company> for CompanyReturn {
    fn from(company: Company) -> Self {
        Self {
            id: company.id,
            team_id: company.team_id,
            name: company.name,
            description: company.description,
            siret: company.siret,
            created_at: company.created_at.to_string(),
            updated_at: company.updated_at.to_string(),
        }
    }
}
