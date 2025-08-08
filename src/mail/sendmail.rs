use std::fs;

use lettre::{
    Transport,
    message::{SinglePart, header},
    transport::smtp::authentication::Credentials,
};

use crate::config::mail_config::MailConfig;

pub async fn send_email(
    mail_config: &MailConfig,
    to_email: &str,
    subject: &str,
    template_path: &str,
    placeholders: &[(String, String)],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut html_template = fs::read_to_string(template_path)?;
    for (key, value) in placeholders {
        html_template = html_template.replace(key.as_str(), value);
    }
    let email = lettre::Message::builder()
        .from(mail_config.smtp_from_address.parse()?)
        .to(to_email.parse()?)
        .subject(subject)
        .header(header::ContentType::TEXT_HTML)
        .singlepart(
            SinglePart::builder()
                .header(header::ContentType::TEXT_HTML)
                .body(html_template),
        )?;
    let creds = Credentials::new(
        mail_config.smtp_username.clone(),
        mail_config.smtp_password.clone(),
    );
    let mailer = lettre::SmtpTransport::starttls_relay(&mail_config.smtp_server)?
        .credentials(creds)
        .port(mail_config.smtp_port)
        .build();
    let result = mailer.send(&email);
    match result {
        Ok(_) => println!("Email sent successfully to {to_email}"),
        Err(e) => println!("Failed to send email to {to_email}: {:?}", e),
    }
    Ok(())
}
