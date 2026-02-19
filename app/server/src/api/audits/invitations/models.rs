use base64::Engine;
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use shared::encryption::{decrypt, encrypt};
use utoipa::ToSchema;
use validator::Validate;

use super::error::AuditInvitationError;

#[derive(
    Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Insertable, Selectable, ToSchema,
)]
#[diesel(table_name = crate::schema::audits_invitations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(rename_all = "camelCase")]
pub struct AuditInvitation {
    pub id: String,
    pub audit_id: String,
    pub client_email: String,
    pub token: String,
    pub expires_at: NaiveDateTime,
    pub is_canceled: bool,
    pub is_used: bool,
    pub used_at: Option<NaiveDateTime>,
    pub created_by: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl AuditInvitation {
    #[must_use]
    pub fn new(
        audit_id: String,
        client_email: String,
        token: String,
        expires_at: NaiveDateTime,
        created_by: String,
    ) -> Self {
        let now = Utc::now().naive_utc();
        Self {
            id: ulid::Ulid::new().to_string(),
            audit_id,
            client_email,
            token,
            expires_at,
            is_canceled: false,
            is_used: false,
            used_at: None,
            created_by,
            created_at: now,
            updated_at: now,
        }
    }

    #[must_use]
    pub fn is_expired(&self) -> bool {
        Utc::now().naive_utc() > self.expires_at
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateAuditInvitationPayload {
    #[validate(email)]
    #[schema(example = "client@example.com")]
    pub client_email: String,

    #[serde(
        deserialize_with = "crate::utils::datetime_serde::deserialize_optional_naive_datetime"
    )]
    pub expires_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditInvitationToken {
    pub audit_id: String,
    pub client_email: String,
    pub expires_at: DateTime<Utc>,
}

impl AuditInvitationToken {
    #[must_use]
    pub fn new(audit_id: String, client_email: String) -> Self {
        let expires_at = Utc::now() + Duration::hours(24);
        Self {
            audit_id,
            client_email,
            expires_at,
        }
    }

    pub fn encrypt(&self, key: &[u8]) -> Result<String, AuditInvitationError> {
        let aes_key = Self::derive_aes_key(key);
        let json = serde_json::to_string(self)?;
        let (ciphertext, nonce) = encrypt(&aes_key, json.as_bytes())?;
        let mut combined = nonce;
        combined.extend_from_slice(&ciphertext);
        let encoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&combined);
        Ok(encoded)
    }

    pub fn decrypt(token: &str, key: &[u8]) -> Result<Self, AuditInvitationError> {
        let aes_key = Self::derive_aes_key(key);
        let combined = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(token)?;
        if combined.len() < 12 {
            return Err(AuditInvitationError::TokenInvalid(
                "Token too short".to_string(),
            ));
        }
        let (nonce, ciphertext) = combined.split_at(12);
        let plaintext = decrypt(&aes_key, nonce, ciphertext)?;
        let token_data: Self = serde_json::from_slice(&plaintext)?;
        if token_data.is_expired() {
            return Err(AuditInvitationError::Expired);
        }
        Ok(token_data)
    }

    fn derive_aes_key(key: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(key);
        let result = hasher.finalize();
        let mut aes_key = [0u8; 32];
        aes_key.copy_from_slice(&result);
        aes_key
    }

    #[must_use]
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AcceptAuditInvitationPayload {
    #[schema(example = "johndoe")]
    pub username: Option<String>,

    #[schema(example = "John")]
    pub first_name: Option<String>,

    #[schema(example = "Doe")]
    pub last_name: Option<String>,

    #[validate(length(min = 8))]
    #[schema(example = "password123")]
    pub password: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum AcceptInvitationResponse {
    AccountCreated {
        user_id: String,
        audit_id: String,
        jwt: String,
    },
    AccountExists {
        audit_id: String,
    },
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CheckInvitationResponse {
    pub audit_name: String,
    pub audit_id: String,
    pub client_email: String,
    pub needs_account_creation: bool,
    pub expires_at: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuditInvitationReturn {
    pub id: String,
    pub audit_id: String,
    pub client_email: String,
    pub expires_at: NaiveDateTime,
    pub is_canceled: bool,
    pub is_used: bool,
    pub created_at: NaiveDateTime,
}

impl From<AuditInvitation> for AuditInvitationReturn {
    fn from(inv: AuditInvitation) -> Self {
        Self {
            id: inv.id,
            audit_id: inv.audit_id,
            client_email: inv.client_email,
            expires_at: inv.expires_at,
            is_canceled: inv.is_canceled,
            is_used: inv.is_used,
            created_at: inv.created_at,
        }
    }
}
