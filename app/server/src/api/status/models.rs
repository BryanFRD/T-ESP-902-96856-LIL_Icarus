use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(example = "Up")]
pub enum StatusEnum {
    Up,
    Maintenance,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Status {
    pub status: StatusEnum,
    #[schema(example = "Up and running!")]
    pub message: String,
}

impl Status {
    pub fn new(status: StatusEnum, message: String) -> Self {
        Self { status, message }
    }
}
