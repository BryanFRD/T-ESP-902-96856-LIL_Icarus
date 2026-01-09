use resend_rs::{Resend, types::CreateEmailResponse};

use crate::mails::services::send_mail;

const PASSWORD_RESET_TEMPLATE: &str = include_str!("../../templates/mails/password-reset.html");

pub async fn send_reset_password_mail(
    token: String,
    first_name: String,
    email: String,
    resend: &Resend,
) -> Result<CreateEmailResponse, resend_rs::Error> {
    let env = std::env::var("VAULT_ENV").unwrap_or("local".to_string());

    let url = match env.as_str() {
        "prod" => format!("https://app.icarus-cyber.fr/reset-password?token={token}"),
        "dev" => format!("https://app-dev.icarus-cyber.fr/reset-password?token={token}"),
        _ => format!("http://localhost:8080/reset-password?token={token}"),
    };

    let mail = PASSWORD_RESET_TEMPLATE
        .replace("{{first_name}}", &first_name)
        .replace("{{reset_url}}", &url)
        .replace("{{email}}", &email);

    send_mail(
        &[email],
        "Password reset request - Icarus".to_string(),
        mail,
        resend,
    )
    .await
}
