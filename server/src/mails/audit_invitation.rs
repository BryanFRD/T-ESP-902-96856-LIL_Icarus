use chrono::Datelike;
use resend_rs::{Resend, types::CreateEmailResponse};

use crate::mails::services::send_mail;

const AUDIT_INVITATION_TEMPLATE: &str = include_str!("../../templates/mails/audit-invitation.html");

pub async fn send_audit_invitation_mail(
    token: String,
    audit_name: String,
    audit_id: String,
    client_email: String,
    resend: &Resend,
) -> Result<CreateEmailResponse, resend_rs::Error> {
    let env = std::env::var("VAULT_ENV").unwrap_or("local".to_string());

    let url = match env.as_str() {
        "prod" => format!("https://app.icarus-cyber.fr/audits/{audit_id}/invite/{token}"),
        "dev" => format!("https://app-dev.icarus-cyber.fr/audits/{audit_id}/invite/{token}"),
        _ => format!("http://localhost:3000/audits/{audit_id}/invite/{token}"),
    };

    let year = chrono::Utc::now().year().to_string();

    let mail = AUDIT_INVITATION_TEMPLATE
        .replace("{{logo_url}}", "https://icarus-cyber.fr/logo.png")
        .replace("{{site_name}}", "Icarus")
        .replace("{{audit_name}}", &audit_name)
        .replace("{{invite_url}}", &url)
        .replace("{{support_email}}", "support@icarus-cyber.fr")
        .replace("{{year}}", &year);

    send_mail(
        &[client_email],
        format!("Invitation to audit: {audit_name}"),
        mail,
        resend,
    )
    .await
}
