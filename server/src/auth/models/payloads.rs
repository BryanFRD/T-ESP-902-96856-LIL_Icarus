use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Clone, Debug, Validate, ToSchema)]
pub struct LoginPayload {
    #[validate(email)]
    #[schema(example = "johndoe@example.com")]
    pub email: String,
    #[schema(example = "password123")]
    pub password: String,
}

#[derive(Deserialize, Clone, Debug, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterPayload {
    #[schema(example = "johndoe")]
    pub username: String,
    #[schema(example = "John")]
    pub first_name: String,
    #[schema(example = "Doe")]
    pub last_name: String,

    #[validate(email(message = "Invalid email format"))]
    #[schema(example = "johndoe@example.com")]
    pub email: String,
    #[schema(example = "password123")]
    pub password: String,
}
