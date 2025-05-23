use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Secrets {
    #[serde(rename = "DATABASE_URL")]
    pub database_url: String,
    #[serde(rename = "JWT_SECRET")]
    pub jwt_secret: String,
}
