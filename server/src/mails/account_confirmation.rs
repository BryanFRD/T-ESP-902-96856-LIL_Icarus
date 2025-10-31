use chrono::Datelike;
use resend_rs::{Resend, types::CreateEmailResponse};

use crate::mails::services::send_mail;

const CONFIRMATION_TEMPLATE: &str = include_str!("../../templates/mails/account-confirmation.html");

pub async fn send_confirmation_mail(
    token: String,
    first_name: String,
    email: String,
    resend: &Resend,
) -> Result<CreateEmailResponse, resend_rs::Error> {
    let env = std::env::var("VAULT_ENV").unwrap_or("local".to_string());

    let url = match env.as_str() {
        "prod" => format!("https://app.icarus-cyber.fr/confirm-email?token={token}"),
        "dev" => format!("https://app-dev.icarus-cyber.fr/confirm-email?token={token}"),
        _ => format!("http://localhost:8080/confirm-email?token={token}"),
    };

    let year = chrono::Utc::now().year().to_string();

    let mail = CONFIRMATION_TEMPLATE
        .replace("{{logo_url}}", "https://icarus-cyber.fr/logo.png")
        .replace("{{site_name}}", "Icarus")
        .replace("{{first_name}}", &first_name)
        .replace("{{confirm_url}}", &url)
        .replace("{{expiry_hours}}", "0.17")
        .replace("{{support_email}}", "support@icarus-cyber.fr")
        .replace("{{year}}", &year);

    send_mail(
        &[email],
        "Confirm your email - Icarus".to_string(),
        mail,
        resend,
    )
    .await
}
