use crate::application::error::AppResult;

pub trait EmailValidator {
    fn validate(&self, email: &String) -> AppResult<()>;
}

pub trait PasswordValidator {
    fn validate(&self, password: &String, email: &String) -> AppResult<()>;
}

#[async_trait::async_trait]
pub trait PasswordHasher {
    async fn hash(&self, password: &String) -> AppResult<String>;
    async fn compare_password(&self, password: &String, hash: &String) -> AppResult<bool>;
}
