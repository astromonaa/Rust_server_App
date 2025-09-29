use crate::services::auth_service::AuthService;
pub struct AuthController {
    service: AuthService,
}


impl AuthController {
    pub fn new(service: AuthService) -> AuthController {
        Self { service }
    }
}

