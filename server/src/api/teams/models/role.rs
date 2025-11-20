use diesel::{
    backend::Backend,
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    serialize::{Output, ToSql},
    sql_types::Text,
};
use serde::Serialize;

#[derive(Debug, Clone, Serialize, FromSqlRow, AsExpression)]
#[diesel(sql_type = Text)]
pub enum TeamRole {
    Owner,
    Moderator,
    Member,
    Client,
}

impl<DB: Backend> FromSql<Text, DB> for TeamRole
where
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: <DB as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        match String::from_sql(bytes)?.as_str() {
            "owner" => Ok(TeamRole::Owner),
            "moderator" => Ok(TeamRole::Moderator),
            "member" => Ok(TeamRole::Member),
            "client" => Ok(TeamRole::Client),
            _ => Err("Unrecognized role".into()),
        }
    }
}

impl<DB: Backend> ToSql<Text, DB> for TeamRole
where
    str: ToSql<Text, DB>,
{
    fn to_sql(&self, out: &mut Output<DB>) -> diesel::serialize::Result {
        match self {
            TeamRole::Owner => "owner".to_sql(out),
            TeamRole::Moderator => "moderator".to_sql(out),
            TeamRole::Member => "member".to_sql(out),
            TeamRole::Client => "client".to_sql(out),
        }
    }
}
