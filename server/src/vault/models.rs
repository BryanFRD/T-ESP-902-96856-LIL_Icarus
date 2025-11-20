use serde::{Deserialize, Serialize};

use crate::api::status::StatusEnum;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Secrets {
    #[serde(rename = "DATABASE_URL")]
    pub database_url: String,
    #[serde(rename = "JWT_SECRET")]
    pub jwt_secret: String,
    #[serde(rename = "STATUS")]
    pub status: StatusEnum,
    #[serde(rename = "STATUS_MESSAGE")]
    pub status_message: String,
    #[serde(rename = "ORIGIN_CORS")]
    pub origin_cors: String,
    #[serde(rename = "RESEND_API_KEY")]
    pub resend_api_key: String,
}
