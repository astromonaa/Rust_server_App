use crate::repository::auth_repository::DbAuthRepository;

pub struct AuthService {
    repository: DbAuthRepository,
}


impl AuthService {
    pub fn new(repository: DbAuthRepository) -> AuthService {
        Self { repository }
    }
}

