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
    pub user_id: i32,
    pub is_anonymous: bool,
}

impl DBChatRepository {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self { connection }
    }

    fn add_user_id_filter(filter: ChatFilter, condition: Condition) -> Condition {
        if filter.is_anonymous {
            condition.add(Column::AnonymousUserId.eq(filter.user_id))
        } else {
            condition.add(Column::UserId.eq(filter.user_id))
        }
    }

    pub async fn create_chat(&self, user_id: Option<i32>, anonymous_user_id: Option<i32>, is_anonymous: bool) -> Result<Model> {
        let new_chat = ActiveModel {
            user_id: Set(user_id),
            anonymous_user_id: Set(anonymous_user_id),
            is_anonymous: Set(is_anonymous),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };

        let saved_chat = new_chat.insert(&*self.connection).await?;
        Ok(saved_chat)
    }

    pub async fn get_chat(&self, filter: ChatFilter) -> Result<Option<Model>> {
        let mut condition = Condition::all();

        if let Some(id) = filter.id {
            condition = condition.add(Column::Id.eq(id));
        }

        let chat = Entity::find()
            .filter(condition)
            .one(&*self.connection)
            .await?;

        Ok(chat)
    }

    pub async fn get_chats(&self, filter: ChatFilter) -> Result<Vec<Model>> {
        let mut condition = Condition::all();

        condition = DBChatRepository::add_user_id_filter(filter, condition);

        let chats = Entity::find()
            .filter(condition)
            .all(&*self.connection)
            .await?;

        Ok(chats)
    }

    pub async fn get_chats_by_user_and_anonymous(
        &self,
        user_id: Option<i32>,
        anonymous_user_id: Option<i32>,
    ) -> Result<Vec<Model>> {
        let mut condition = Condition::any();

        if let Some(uid) = user_id {
            condition = condition.add(Column::UserId.eq(uid));
        }

        if let Some(anon_id) = anonymous_user_id {
            condition = condition.add(Column::AnonymousUserId.eq(anon_id));
        }

        let chats = Entity::find()
            .filter(condition)
            .all(&*self.connection)
            .await?;

        Ok(chats)
    }

    pub async fn create_message(&self, chat_id: i32, text: String, is_from_user: bool) -> Result<MessageModel> {
        let new_message = MessageActiveModel {
            chat_id: Set(chat_id),
            text: Set(text),
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

    pub async fn get_messages_by_chat_ids(&self, chat_ids: Vec<i32>) -> Result<Vec<MessageModel>> {
        use crate::db::entities::message::Column as MessageColumn;
        
        let messages = MessageEntity::find()
            .filter(MessageColumn::ChatId.is_in(chat_ids))
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
