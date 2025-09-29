use std::fmt;
use std::fmt::Formatter;

pub enum MailErrors {
    RenderTemplateError,
    MessageBuildError,
    SendMessageError,
    ParseFromError,
    ParseToError,
    ConfigLoadError
}

impl fmt::Display for MailErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MailErrors::RenderTemplateError => write!(f, "RenderTemplateError"),
            MailErrors::MessageBuildError => write!(f, "MessageBuildError"),
            MailErrors::SendMessageError => write!(f, "SendMessageError"),
            MailErrors::ParseFromError => write!(f, "ParseFromError"),
            MailErrors::ParseToError => write!(f, "ParseToError"),
            MailErrors::ConfigLoadError => write!(f, "ConfigLoadError")
        }
    }
}