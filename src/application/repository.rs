use uuid::Uuid;

use crate::application::{dao::UserDAO, error::AppResult};

#[async_trait::async_trait]
pub trait UserRepository {
    async fn next_id(&self) -> AppResult<Uuid>;
    async fn id_exists(&self, id: &Uuid) -> AppResult<bool>;
    async fn email_exists(&self, email: &String) -> AppResult<bool>;
    async fn by_id(&self, id: &Uuid) -> AppResult<UserDAO>;
    async fn by_email(&self, email: &String) -> AppResult<UserDAO>;
    async fn save(&mut self, user: &UserDAO) -> AppResult<()>;
}
