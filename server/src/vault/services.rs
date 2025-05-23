use tracing::info;
use vaultrs::{
    auth::approle::login,
    client::{Client, VaultClient, VaultClientSettingsBuilder},
    kv2,
};

use super::models::Secrets;

pub async fn get_secrets() -> Result<Secrets, String> {
    #[cfg(not(debug_assertions))]
    let _ = ();

    #[cfg(debug_assertions)]
    {
        dotenvy::dotenv().ok();
    }

    let address = std::env::var("VAULT_ADDRESS").expect("VAULT_ADDRESS not set");
    let role_id = std::env::var("VAULT_ROLE_ID").expect("VAULT_ROLE_ID not set");
    let secret_id = std::env::var("VAULT_SECRET_ID").expect("VAULT_SECRET_ID not set");

    let settings = match VaultClientSettingsBuilder::default()
        .address(address)
        .build()
    {
        Ok(settings) => settings,
        Err(e) => {
            tracing::error!("Failed to create Vault client settings: {}", e);
            return Err("Failed to create Vault client settings".to_string());
        }
    };

    let mut client = match VaultClient::new(settings) {
        Ok(client) => client,
        Err(e) => {
            tracing::error!("Failed to create Vault client: {}", e);
            return Err("Failed to create Vault client".to_string());
        }
    };

    // Perform AppRole login
    let login = match login(&client, "approle", role_id.as_str(), secret_id.as_str()).await {
        Ok(login) => login,
        Err(e) => {
            tracing::error!("Vault login failed: {}", e);
            return Err("Login failed".to_string());
        }
    };

    client.set_token(&login.client_token);

    let env = std::env::var("VAULT_ENV").unwrap_or("local".to_string());

    let secrets: Secrets =
        match kv2::read(&client, "secret", format!("env/{}/app", env).as_str()).await {
            Ok(secrets) => secrets,
            Err(e) => {
                tracing::error!("Failed to read secret: {:?}", e);
                return Err("Failed to read secret".to_string());
            }
        };

    info!("Successfully retrieved secrets from Vault");
    Ok(secrets)
}
