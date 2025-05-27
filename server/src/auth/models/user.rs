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

use crate::auth::services::hash_password;

use super::RegisterPayload;

#[derive(Serialize, Deserialize, Queryable, Insertable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub role: UserRole,
    #[serde(skip_serializing)]
    pub password_hash: String,
    #[serde(skip_serializing)]
    pub is_connected: bool,
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

#[derive(Serialize, Deserialize, Clone, Debug, AsExpression, FromSqlRow, PartialEq)]
#[diesel(sql_type = Text)]
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
