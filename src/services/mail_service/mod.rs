pub mod types;

use std::collections::HashMap;
use lettre::{SmtpTransport, transport::smtp::authentication::Credentials, message::{Message, Mailbox, header::ContentType}, Transport};
use crate::configuration::auth_configuration::AuthConfiguration;
use crate::configuration::global_configuration::GlobalConfiguration;
use lettre::message::SinglePart;
use crate::services::mail_service::types::MailErrors;
use crate::utils::render_template::render_template;

pub struct MailService {
    mailer: SmtpTransport,
    host: String,
}

impl MailService {
    pub fn new() -> anyhow::Result<MailService> {
        let config = AuthConfiguration::load()?;

        let credentials = Credentials::new(config.smtp_user, config.smtp_password);

        let mailer = SmtpTransport::starttls_relay(&config.smtp_host)?
            .credentials(credentials)
            .build();

        Ok(MailService { mailer, host: String::from("http://localhost:5000") })
    }

    pub fn send_activation_link(&self, to: &str, activation_link: &str) -> Result<(), MailErrors> {
        let global_config = GlobalConfiguration::load().map_err(|_|MailErrors::ConfigLoadError)?;
        let link = format!("{}/api/users/activate/{}", global_config.domain, activation_link);

        let mut vars = HashMap::new();
        vars.insert("activation_link", link.as_str());

        let html_body = render_template("templates/email_activate.html", &vars).map_err(|_| MailErrors::RenderTemplateError)?;

        let email = Message::builder()
            .from("Coctail <noreply@coctail.com>".parse::<Mailbox>().map_err(|_| MailErrors::ParseFromError)?)
            .to(to.parse::<Mailbox>().map_err(|_| MailErrors::ParseToError)?)
            .subject("Account activation")
            .singlepart(
                SinglePart::builder()
                    .header(ContentType::TEXT_HTML)
                    .body(html_body)
            ).map_err(|_| MailErrors::MessageBuildError)?;

        self.mailer.send(&email).map_err(|_| MailErrors::SendMessageError)?;
        Ok(())
    }
}