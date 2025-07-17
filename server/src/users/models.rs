use std::fmt::Display;

use diesel::{
    backend::Backend,
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    prelude::*,
    serialize::{Output, ToSql},
    sql_types::Text,
};
use serde::{Deserialize, Serialize};
use ulid::Ulid;
use utoipa::ToSchema;
use validator::Validate;

use crate::auth::hash_password;

use crate::auth::RegisterPayload;

#[derive(
    Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Selectable, ToSchema, Identifiable,
)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[schema(example = "01F8MECHZX3TBDSZ7XK4F5G9ZQ")]
    pub id: String,
    #[schema(example = "johndoe")]
    pub username: String,
    #[schema(example = "John")]
    pub first_name: String,
    #[schema(example = "Doe")]
    pub last_name: String,
    #[schema(example = "johndoe@example.com")]
    pub email: String,
    pub role: UserRole,
    #[serde(skip_serializing)]
    pub password_hash: String,
    #[serde(skip_serializing)]
    pub is_connected: bool,
    pub avatar_url: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub last_connected_at: chrono::NaiveDateTime,
}

impl User {
    pub fn new(
        username: String,
        first_name: String,
        last_name: String,
        email: String,
        role: String,
        password: String,
    ) -> Self {
        Self {
            id: Ulid::new().to_string(),
            username,
            first_name,
            last_name,
            email,
            role: role.into(),
            password_hash: hash_password(password.as_str()),
            is_connected: false,
            avatar_url: None,
            created_at: chrono::Local::now().naive_utc(),
            updated_at: chrono::Local::now().naive_utc(),
            last_connected_at: chrono::Local::now().naive_utc(),
        }
    }

    pub fn verify_password(&self, password: &str) -> bool {
        hash_password(password) == self.password_hash
    }
}

impl From<RegisterPayload> for User {
    fn from(payload: RegisterPayload) -> Self {
        Self::new(
            payload.username,
            payload.first_name,
            payload.last_name,
            payload.email,
            "pentester".to_string(),
            payload.password,
        )
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, AsExpression, FromSqlRow, PartialEq, ToSchema)]
#[diesel(sql_type = Text)]
#[schema(example = "Pentester")]
pub enum UserRole {
    Admin,
    Pentester,
    Client,
}

impl<DB: Backend> FromSql<Text, DB> for UserRole
where
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: <DB as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        match String::from_sql(bytes)?.as_str() {
            "admin" => Ok(UserRole::Admin),
            "pentester" => Ok(UserRole::Pentester),
            "client" => Ok(UserRole::Client),
            _ => Err("Unrecognized role".into()),
        }
    }
}

impl<DB: Backend> ToSql<Text, DB> for UserRole
where
    str: ToSql<Text, DB>,
{
    fn to_sql(&self, out: &mut Output<DB>) -> diesel::serialize::Result {
        match self {
            UserRole::Admin => "admin".to_sql(out),
            UserRole::Pentester => "pentester".to_sql(out),
            UserRole::Client => "client".to_sql(out),
        }
    }
}

impl Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Admin => write!(f, "admin"),
            UserRole::Pentester => write!(f, "pentester"),
            UserRole::Client => write!(f, "client"),
        }
    }
}

impl From<String> for UserRole {
    fn from(s: String) -> Self {
        match s.as_str() {
            "admin" => UserRole::Admin,
            "pentester" => UserRole::Pentester,
            "client" => UserRole::Client,
            _ => UserRole::Client,
        }
    }
}

#[derive(Deserialize, Clone, Debug, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePayload {
    #[schema(example = "johndoe")]
    pub username: Option<String>,
    #[schema(example = "John")]
    pub first_name: Option<String>,
    #[schema(example = "Doe")]
    pub last_name: Option<String>,

    #[validate(email(message = "Invalid email format"))]
    #[schema(example = "johndoe@example.com")]
    pub email: Option<String>,

    #[schema(example = "current_password123")]
    pub password: Option<String>,
    #[schema(example = "new_password123")]
    pub new_password: Option<String>,
    #[schema(example = "new_password123")]
    pub confirm_password: Option<String>,
}
#[allow(dead_code)]
#[derive(Deserialize, ToSchema)]
pub struct AvatarForm {
    #[schema(format = Binary, content_media_type = "image/*")]
    avatar: String,
}
