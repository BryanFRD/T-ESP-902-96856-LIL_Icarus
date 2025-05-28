use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Clone, Debug, Validate, ToSchema)]
pub struct LoginPayload {
    #[validate(email)]
    #[schema(example = "johndoe@example.com")]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be at least 4 characters long"))]
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
