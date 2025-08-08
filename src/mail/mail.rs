use std::vec;

use crate::{config::mail_config::MailConfig, mail::sendmail::send_email};

fn create_verification_link(base_url: &str, token: &str) -> String {
    format!("{}?token={}", base_url, token)
}

pub async fn send_verification_email(
    mail_config: &MailConfig,
    backend_base_url: &str,
    mail_template_path: &str,
    to_email: &str,
    name: &str,
    token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let subject = "Email Verification";
    let template_path = format!("{}/{}", mail_template_path, "verification-email.html");
    let base_url = format!("{}/auth/verify", backend_base_url);
    let verification_link = create_verification_link(&base_url, token);
    let placeholder = vec![
        ("{{ .Name }}".to_string(), name.to_string()),
        ("{{ .Email }}".to_string(), to_email.to_string()),
        ("{{ .ConfirmationURL }}".to_string(), verification_link),
    ];
    send_email(mail_config, to_email, subject, &template_path, &placeholder).await
}

pub async fn send_welcome_email(
    mail_config: &MailConfig,
    to_email: &str,
    frontend_base_url: &str,
    name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let subject = "Welcome to workspace kit";
    let template_path = format!(
        "{}/{}",
        mail_config.mail_template_path, "welcome-email.html"
    );
    let base_url = frontend_base_url;
    let placeholders = vec![
        ("{{ .Name }}".to_string(), name.to_string()),
        ("{{ .SiteURL }}".to_string(), base_url.to_string()),
    ];

    send_email(
        mail_config,
        to_email,
        &subject,
        &template_path,
        &placeholders,
    )
    .await
}

pub async fn send_password_reset_email(
    mail_config: &MailConfig,
    to_email: &str,
    frontend_base_url: &str,
    name: &str,
    token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let subject = "Password Reset Request";
    let template_path = format!(
        "{}/{}",
        mail_config.mail_template_path, "reset-password.html"
    );
    let base_url = format!("{}/auth/reset-password", frontend_base_url);
    let verification_link = create_verification_link(&base_url, token);
    let placeholders = vec![
        ("{{ .Name }}".to_string(), name.to_string()),
        ("{{ .Email }}".to_string(), to_email.to_string()),
        ("{{ .ConfirmationURL }}".to_string(), verification_link),
    ];
    send_email(
        mail_config,
        to_email,
        subject,
        &template_path,
        &placeholders,
    )
    .await
}

pub async fn send_email_change_notification(
    mail_config: &MailConfig,
    frontend_base_url: &str,
    old_email: &str,
    new_email: &str,
    name: &str,
    token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let subject = "Email Change Notification";
    let template_path = format!(
        "{}/{}",
        mail_config.mail_template_path, "email-change-notification.html"
    );
    let base_url = format!("{}/user/change-email", frontend_base_url);
    let verification_link = create_verification_link(&base_url, token);
    let placeholders = vec![
        ("{{ .Name }}".to_string(), name.to_string()),
        ("{{ .Email }}".to_string(), old_email.to_string()),
        ("{{ .NewEmail }}".to_string(), new_email.to_string()),
        ("{{ .ConfirmationURL }}".to_string(), verification_link),
    ];
    send_email(
        mail_config,
        new_email,
        subject,
        &template_path,
        &placeholders,
    )
    .await
}
