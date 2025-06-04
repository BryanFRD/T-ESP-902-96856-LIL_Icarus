use diesel::{
    Selectable,
    prelude::{Identifiable, Insertable, Queryable},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Queryable,
    Identifiable,
    PartialEq,
    Insertable,
    ToSchema,
    Selectable,
)]
#[diesel(table_name = crate::schema::teams)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct Team {
    #[schema(example = "01F8MECHZX3TBDSZ7XK4F5G9ZQ")]
    pub id: String,
    #[schema(example = "Example Team")]
    pub name: String,
    #[schema(example = "This is an example team description.")]
    pub description: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Team {
    pub fn new(name: String, description: String) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Team {
            id: ulid::Ulid::new().to_string(),
            name,
            description,
            created_at: now,
            updated_at: now,
        }
    }
}
