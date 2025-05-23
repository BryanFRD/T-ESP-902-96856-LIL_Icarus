use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Clone, Debug, Validate)]
pub struct LoginPayload {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Clone, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RegisterPayload {
    pub username: String,
    pub first_name: String,
    pub last_name: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    pub password: String,
}
