use resend_rs::{
    Resend,
    types::{CreateEmailBaseOptions, CreateEmailResponse},
};
use tracing::info;

pub async fn send_mail(
    to: &[String],
    subject: String,
    body: String,
    resend: &Resend,
) -> Result<CreateEmailResponse, resend_rs::Error> {
    let email = CreateEmailBaseOptions::new("Icarus <no-reply@icarus-cyber.fr>", to, subject)
        .with_html(&body);
    info!("{resend:?}");
    resend.emails.send(email).await
}
