use crate::db::entities::user::Model;

#[derive(Debug, Clone, serde::Serialize)]
pub struct UserDto {
    pub id: i32,
    pub role: String,
    pub email: String,
    pub is_activated: bool,
    pub activation_link: String,
}

impl From<Model> for UserDto {
    fn from(user: Model) -> Self {
        Self {
            id: user.id,
            email: user.email,
            role: user.role,
            is_activated: user.is_activated,
            activation_link: user.activation_link,
        }
    }
}