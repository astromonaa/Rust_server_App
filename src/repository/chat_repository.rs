use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter, Set, DbErr
};
use crate::db::entities::chat::{ActiveModel, Column, Entity, Model};
use crate::db::entities::message::{
    ActiveModel as MessageActiveModel,
    Entity as MessageEntity,
    Model as MessageModel,
};
use anyhow::Result;
use chrono::Utc;
use std::sync::Arc;

#[derive(Clone)]
pub struct DBChatRepository {
    pub connection: Arc<DatabaseConnection>,
}

#[derive(Default)]
pub struct ChatFilter {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub anonymous_user_id: Option<i32>,
    pub is_anonymous: Option<bool>,
}

impl DBChatRepository {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self { connection }
    }

    pub async fn create_chat(&self, user_id: Option<i32>, anonymous_user_id: i32, is_anonymous: bool) -> Result<Model> {
        let new_chat = ActiveModel {
            user_id: Set(user_id),
            anonymous_user_id: Set(anonymous_user_id),
            is_anonymous: Set(is_anonymous),
            created_at: Set(Utc::now()),
            ..Default::default()
        };

        let saved_chat = new_chat.insert(&*self.connection).await?;
        Ok(saved_chat)
    }

    pub async fn get_chat(&self, filter: ChatFilter) -> Result<Option<Model>> {
        let mut condition = Condition::all();

        if filter.id.is_none() && filter.user_id.is_none() && filter.anonymous_user_id.is_none() {
            return Err(DbErr::Custom("At least one filter parameter must be provided".into()))?;
        }

        if let Some(id) = filter.id {
            condition = condition.add(Column::Id.eq(id));
        }

        if let Some(user_id) = filter.user_id {
            condition = condition.add(Column::UserId.eq(user_id));
        }

        if let Some(anonymous_user_id) = filter.anonymous_user_id {
            condition = condition.add(Column::AnonymousUserId.eq(anonymous_user_id));
        }

        if let Some(is_anonymous) = filter.is_anonymous {
            condition = condition.add(Column::IsAnonymous.eq(is_anonymous));
        }

        let chat = Entity::find()
            .filter(condition)
            .one(&*self.connection)
            .await?;

        Ok(chat)
    }

    pub async fn get_chats(&self, filter: ChatFilter) -> Result<Vec<Model>> {
        let mut condition = Condition::all();

        if let Some(user_id) = filter.user_id {
            condition = condition.add(Column::UserId.eq(user_id));
        }

        if let Some(is_anonymous) = filter.is_anonymous {
            condition = condition.add(Column::IsAnonymous.eq(is_anonymous));
        }

        let chats = Entity::find()
            .filter(condition)
            .all(&*self.connection)
            .await?;

        Ok(chats)
    }

    pub async fn create_message(&self, chat_id: i32, content: String, is_from_user: bool) -> Result<MessageModel> {
        let new_message = MessageActiveModel {
            chat_id: Set(chat_id),
            content: Set(content),
            is_from_user: Set(is_from_user),
            is_read: Set(false),
            created_at: Set(Utc::now()),
            ..Default::default()
        };

        let saved_message = new_message.insert(&*self.connection).await?;
        Ok(saved_message)
    }

    pub async fn get_messages(&self, chat_id: i32) -> Result<Vec<MessageModel>> {
        let messages = MessageEntity::find()
            .filter(crate::db::entities::message::Column::ChatId.eq(chat_id))
            .all(&*self.connection)
            .await?;

        Ok(messages)
    }

    pub async fn mark_messages_as_read(&self, chat_id: i32) -> Result<()> {
        use sea_orm::sea_query::Expr;
        use sea_orm::EntityTrait;

        MessageEntity::update_many()
            .col_expr(
                crate::db::entities::message::Column::IsRead,
                Expr::value(true)
            )
            .filter(crate::db::entities::message::Column::ChatId.eq(chat_id))
            .exec(&*self.connection)
            .await?;

        Ok(())
    }
}
