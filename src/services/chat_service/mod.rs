pub mod types;

use crate::repository::chat_repository::{DBChatRepository, ChatFilter};
use crate::db::entities::chat::Model as ChatModel;
use crate::db::entities::message::Model as MessageModel;
use anyhow::Result;
use crate::services::chat_service::types::ChatErrors;

#[derive(Clone)]
pub struct ChatService {
    repository: DBChatRepository,
}

impl ChatService {
    pub fn new(repository: DBChatRepository) -> Self {
        Self { repository }
    }

    pub async fn create_chat(&self, user_id: Option<i32>, anonymous_user_id: Option<i32>, is_anonymous: bool) -> Result<ChatModel, ChatErrors> {
        let anonymous_user_id = if is_anonymous { anonymous_user_id } else { None };
        let chat = self.repository.create_chat(user_id, anonymous_user_id, is_anonymous)
            .await
            .map_err(|e| ChatErrors::CreationError(e.to_string()))?;

        Ok(chat)
    }

    pub async fn get_chat(&self, chat_id: i32) -> Result<Option<ChatModel>, ChatErrors> {
        let chat = self.repository.get_chat(ChatFilter {
            id: Some(chat_id),
            ..Default::default()
        })
            .await
            .map_err(|e| ChatErrors::FetchError(e.to_string()))?;

        Ok(chat)
    }

    pub async fn get_chats_by_user_and_anonymous(
        &self,
        user_id: Option<i32>,
        anonymous_user_id: Option<i32>,
    ) -> Result<Vec<ChatModel>, ChatErrors> {
        let chats = self.repository
            .get_chats_by_user_and_anonymous(user_id, anonymous_user_id)
            .await
            .map_err(|e| ChatErrors::FetchError(e.to_string()))?;

        Ok(chats)
    }

    pub async fn send_message(&self, chat_id: i32, text: String, is_from_user: bool) -> Result<MessageModel, ChatErrors> {
        let chat = self.get_chat(chat_id).await?;

        if chat.is_none() {
            return Err(ChatErrors::ChatNotFound);
        }

        let message = self.repository.create_message(chat_id, text, is_from_user)
            .await
            .map_err(|e| ChatErrors::MessageCreationError(e.to_string()))?;

        Ok(message)
    }

    pub async fn get_messages_by_chat_ids(&self, chat_ids: Vec<i32>) -> Result<Vec<MessageModel>, ChatErrors> {
        let messages = self.repository
            .get_messages_by_chat_ids(chat_ids)
            .await
            .map_err(|e| ChatErrors::FetchError(e.to_string()))?;

        Ok(messages)
    }

    pub async fn mark_messages_as_read(&self, chat_id: i32) -> Result<(), ChatErrors> {
        self.repository.mark_messages_as_read(chat_id)
            .await
            .map_err(|e| ChatErrors::UpdateError(e.to_string()))?;

        Ok(())
    }
}
