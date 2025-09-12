use resend_rs::{types::{CreateEmailBaseOptions}, Error, Resend};

pub async fn send_mail(to: &[String], subject: String, body: String, resend: Resend) -> Result<(), Error> {    
    let email = CreateEmailBaseOptions::new("Icarus <no-reply@icarus-cyber.fr>", to, subject).with_html(&body);
    

    let _email = resend.emails.send(email).await?;

    Ok(())
}
