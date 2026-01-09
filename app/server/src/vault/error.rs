use thiserror::Error;
use vaultrs::{client::VaultClientSettingsBuilderError, error::ClientError};

#[derive(Debug, Error)]
pub enum VaultError {
    #[error("Failed to create Vault Client Settings: {0}")]
    SettingCreationError(#[from] VaultClientSettingsBuilderError),
    #[error("Got a Vault Client error: {0}")]
    ClientError(#[from] ClientError),
}
