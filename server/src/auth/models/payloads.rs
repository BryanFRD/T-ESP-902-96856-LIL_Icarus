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

#[derive(Deserialize, Clone, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResetPasswordRequestPayload {
    #[schema(example = "johndoe@example.com")]
    pub reset_email: String,
}

#[derive(Deserialize, Clone, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResetPasswordPayload {
    #[schema(
        example = "5e0bf8751c9bcfe29632c0bce5dc8e8f4813122051ea13bfd026bb4766db61464120e7fe9d594743ce277946cb6d4a8355b712880b4ed1f8a657bd1858918ede"
    )]
    pub token: String,
    #[schema(example = "password123")]
    pub new_password: String,
}
