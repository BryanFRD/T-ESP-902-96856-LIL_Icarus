use tracing::info;
use vaultrs::{
    auth::approle::login,
    client::{Client, VaultClient, VaultClientSettingsBuilder},
    kv2,
};

use super::{VaultError, Secrets};

pub async fn get_secrets() -> Result<Secrets, VaultError> {
    #[cfg(debug_assertions)]
    {
        dotenvy::dotenv().ok();
    }

    let address = std::env::var("VAULT_ADDRESS").expect("VAULT_ADDRESS not set");
    let role_id = std::env::var("VAULT_ROLE_ID").expect("VAULT_ROLE_ID not set");
    let secret_id = std::env::var("VAULT_SECRET_ID").expect("VAULT_SECRET_ID not set");

    let settings = VaultClientSettingsBuilder::default()
        .address(address)
        .build()?;

    let mut client = VaultClient::new(settings)?;

    // Perform AppRole login
    let login = login(&client, "approle", role_id.as_str(), secret_id.as_str()).await?;

    client.set_token(&login.client_token);

    let env = std::env::var("VAULT_ENV").unwrap_or("local".to_string());

    let secrets: Secrets = kv2::read(&client, "secret", format!("env/{env}/app").as_str()).await?;

    info!("Successfully retrieved secrets from Vault");
    Ok(secrets)
}
