use std::fmt;

#[derive(Debug)]
pub enum ChatErrors {
    CreationError(String),
    FetchError(String),
    ChatNotFound,
    MessageCreationError(String),
    UpdateError(String),
}

impl fmt::Display for ChatErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChatErrors::CreationError(msg) => write!(f, "Failed to create chat: {}", msg),
            ChatErrors::FetchError(msg) => write!(f, "Failed to fetch chat: {}", msg),
            ChatErrors::ChatNotFound => write!(f, "Chat not found"),
            ChatErrors::MessageCreationError(msg) => write!(f, "Failed to create message: {}", msg),
            ChatErrors::UpdateError(msg) => write!(f, "Failed to update: {}", msg),
        }
    }
}

impl std::error::Error for ChatErrors {}
