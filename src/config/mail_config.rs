use std::env;

#[derive(Debug, Clone)]
pub struct MailConfig {
    pub smtp_server: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_from_address: String,
    pub mail_template_path: String,
}

impl MailConfig {
    pub fn init() -> Self {
        let smtp_server = env::var("SMTP_SERVER").expect("SMTP_SERVER not set in env");
        let smtp_port = env::var("SMTP_PORT")
            .expect("SMTP_PORT not set in env")
            .parse()
            .expect("SMTP_PORT must be a number");
        let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME not set in env");
        let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD not set in env");
        let smtp_from_address =
            env::var("SMTP_FROM_ADDRESS").expect("SMTP_FROM_ADDRESS not set in env");
        let mail_template_path =
            env::var("MAIL_TEMPLATE_PATH").expect("MAIL_TEMPLATE_PATH not set in env");
        MailConfig {
            smtp_server: smtp_server,
            smtp_port: smtp_port,
            smtp_username: smtp_username,
            smtp_password: smtp_password,
            smtp_from_address: smtp_from_address,
            mail_template_path: mail_template_path,
        }
    }
}
