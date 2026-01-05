use crate::db::entities::anonymous_user::{
    ActiveModel as AnonymousUserActiveModel, Model as AnonymousUserModel,
    Entity as AnonymousUserEntity,
    Column as AnonymousUserColumn
};
use crate::db::entities::user::{ActiveModel, Column, Entity, Model};
use crate::websocket::ws_helpers::BrowserInfo;
use anyhow::Result;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
    Set,
};
use std::sync::Arc;

pub struct DbUserRepository {
    pub connection: Arc<DatabaseConnection>,
}

#[derive(Default)]
pub struct UserFilter {
    pub id: Option<i32>,
    pub email: Option<String>,
    pub activation_link: Option<String>,
    pub password: Option<String>,
    pub is_activated: Option<bool>,
}

impl DbUserRepository {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self { connection }
    }

    pub async fn get_users(&self) -> Result<Vec<Model>> {
        let users = Entity::find().all(&*self.connection).await?;
        Ok(users)
    }

    pub async fn get_user(&self, filter: UserFilter) -> Result<Option<Model>> {
        let mut condition = Condition::all();

        if filter.id.is_none() && filter.email.is_none() && filter.activation_link.is_none() {
            return Err(DbErr::Custom(
                "At least one filter parameter must be provided".into(),
            ))?;
        }

        if let Some(id) = filter.id {
            condition = condition.add(Column::Id.eq(id));
        }

        if let Some(email) = filter.email {
            condition = condition.add(Column::Email.eq(email));
        }

        if let Some(activation_link) = filter.activation_link {
            condition = condition.add(Column::ActivationLink.eq(activation_link));
        }

        let user = Entity::find()
            .filter(condition)
            .one(&*self.connection)
            .await?;

        Ok(user)
    }

    pub async fn create_user(
        &self,
        email: String,
        password: String,
        activation_link: String,
    ) -> Result<Model> {
        // Используем Set для обёртки значений в ActiveValue
        let new_user = ActiveModel {
            email: Set(email),
            password: Set(password),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            activation_link: Set(activation_link),
            ..Default::default()
        };

        let saved_user = new_user.insert(&*self.connection).await?;

        Ok(saved_user)
    }

    pub async fn update_user(&self, user_id: i32, data: UserFilter) -> Result<Model> {
        let user = Entity::find_by_id(user_id).one(&*self.connection).await?;

        let Some(user) = user else {
            return Err(DbErr::RecordNotFound("User not found".into()))?;
        };

        let mut active: ActiveModel = user.into();

        if let Some(email) = data.email {
            active.email = Set(email);
        }
        if let Some(password) = data.password {
            active.password = Set(password);
        }
        if let Some(link) = data.activation_link {
            active.activation_link = Set(link);
        }

        if let Some(is_activated) = data.is_activated {
            active.is_activated = Set(is_activated);
        }

        active.updated_at = Set(Utc::now());

        let updated = active.update(&*self.connection).await?;

        Ok(updated)
    }
    pub async fn get_anonymous_user(&self, user_data: &BrowserInfo) -> Result<Option<AnonymousUserModel>> {
        let anonymous_user = AnonymousUserEntity::find()
            .filter(Condition::all().add(AnonymousUserColumn::DeviceFingerprint.eq(user_data.device_fingerprint.clone())))
            .one(&*self.connection)
            .await?;

        Ok(anonymous_user)
    }

    pub async fn create_anonymous_user(
        &self,
        user_data: BrowserInfo,
    ) -> Result<AnonymousUserModel> {
        let anonymous_user = AnonymousUserActiveModel {
            device_fingerprint: Set(user_data.device_fingerprint),
            user_agent: Set(Some(user_data.user_agent)),
            ip_address: Set(Some(user_data.ip_address)),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        };

        let saved_user = anonymous_user.insert(&*self.connection).await.map_err(|e| {
            println!("Failed to create anonymous user: {}", e);
            e
        })?;

        Ok(saved_user)
    }
}
