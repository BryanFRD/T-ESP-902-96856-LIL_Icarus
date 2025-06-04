use serde::{Deserialize, Serialize};
use ulid::Ulid;

use crate::users::{User, UserRole};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JwtClaims {
    pub sub: Ulid,
    pub email: String,
    pub role: UserRole,
    pub iat: usize,
    pub exp: usize,
}

impl JwtClaims {
    pub fn to_string(&self, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
        jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            self,
            &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()),
        )
    }

    pub fn from_jwt(jwt: &str, secret: &str) -> Self {
        let token_data = jsonwebtoken::decode::<JwtClaims>(
            jwt,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
            &jsonwebtoken::Validation::default(),
        );

        match token_data {
            Ok(data) => data.claims,
            Err(_) => JwtClaims {
                sub: Ulid::nil(),
                email: String::new(),
                role: UserRole::Client,
                iat: 0,
                exp: 0,
            },
        }
    }

    pub fn is_admin(&self) -> bool {
        self.role == UserRole::Admin
    }

    pub fn is_pentester(&self) -> bool {
        self.role == UserRole::Pentester || self.role == UserRole::Admin
    }

    pub fn is_valid(&self) -> bool {
        let now = chrono::Local::now().timestamp() as usize;
        self.iat <= now && self.exp >= now
    }
}

impl From<User> for JwtClaims {
    fn from(user: User) -> Self {
        let ulid = Ulid::from_string(user.id.as_str()).unwrap_or(Ulid::nil());

        Self {
            sub: ulid,
            email: user.email,
            role: user.role,
            iat: chrono::Local::now().timestamp() as usize,
            exp: (chrono::Local::now() + chrono::Duration::days(1)).timestamp() as usize,
        }
    }
}
