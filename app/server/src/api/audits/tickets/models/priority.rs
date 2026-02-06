use diesel::{
    backend::Backend,
    deserialize::{FromSql, FromSqlRow},
    expression::AsExpression,
    serialize::{Output, ToSql},
    sql_types::Text,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Serialize, Deserialize, FromSqlRow, AsExpression, ToSchema, PartialEq, Eq,
)]
#[diesel(sql_type = Text)]
#[serde(rename_all = "UPPERCASE")]
pub enum TicketPriority {
    Low,
    Medium,
    High,
    Critical,
}

impl<DB: Backend> FromSql<Text, DB> for TicketPriority
where
    String: FromSql<Text, DB>,
{
    fn from_sql(bytes: <DB as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
        match String::from_sql(bytes)?.as_str() {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            "critical" => Ok(Self::Critical),
            _ => Err("Unrecognized ticket priority".into()),
        }
    }
}

impl<DB: Backend> ToSql<Text, DB> for TicketPriority
where
    str: ToSql<Text, DB>,
{
    fn to_sql(&self, out: &mut Output<DB>) -> diesel::serialize::Result {
        match self {
            Self::Low => "low".to_sql(out),
            Self::Medium => "medium".to_sql(out),
            Self::High => "high".to_sql(out),
            Self::Critical => "critical".to_sql(out),
        }
    }
}
