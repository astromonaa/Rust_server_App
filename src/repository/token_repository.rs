use sea_orm::{DatabaseConnection, QueryFilter, EntityTrait, ColumnTrait, Set, ActiveModelTrait, ModelTrait};
use crate::db::entities::token::{ Entity as TokenEntity, Column, Model };
use anyhow::Result;
use chrono::Utc;
use crate::db::entities::token::ActiveModel;

pub struct TokenRepository<'a> {
    connection: &'a DatabaseConnection
}

impl<'a> TokenRepository<'a> {
    pub fn new(connection: &'a DatabaseConnection) -> Self {
        Self { connection }
    }

    pub async fn get_token(&self, refresh_token: &str) -> Result<Option<Model>> {
        let token = TokenEntity::find()
            .filter(Column::RefreshToken.eq(refresh_token))
            .one(self.connection)
            .await?;

        Ok(token)
    }

    pub async fn save_token(&self, refresh_token: String, user_id: i32) -> Result<String> {
        let token = ActiveModel {
            refresh_token: Set(refresh_token),
            user_id: Set(user_id),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };

        let new_token = token.insert(self.connection).await?;
        Ok(new_token.refresh_token)
    }

    pub async fn remove_token(&self, refresh_token: &str) -> Result<String> {
        let token = TokenEntity::find()
        .filter(Column::RefreshToken.eq(refresh_token))
        .one(self.connection)
        .await?.unwrap();


        let deleted_token = token.delete(self.connection).await?;
        Ok(deleted_token.rows_affected.to_string())
    }
}