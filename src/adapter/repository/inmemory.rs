use std::collections::HashMap;

use tokio::sync::Mutex;
use uuid::Uuid;

use crate::application::{
    dao::UserDAO,
    error::{AppError, AppResult},
    repository::UserRepository,
};

#[derive(Debug)]
pub struct InMemoryUserRepository {
    store: Mutex<HashMap<Uuid, UserDAO>>,
}

#[async_trait::async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn next_id(&self) -> AppResult<Uuid> {
        Ok(Uuid::new_v4())
    }

    async fn id_exists(&self, id: &Uuid) -> AppResult<bool> {
        match self.store.lock().await.get(id) {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    async fn email_exists(&self, email: &String) -> AppResult<bool> {
        Ok(self.store.lock().await.values().any(|u| u.email == *email))
    }

    async fn by_id(&self, id: &Uuid) -> AppResult<UserDAO> {
        match self.store.lock().await.get(id) {
            Some(user) => Ok(user.clone()),
            None => Err(AppError::NotFound(format!(
                "пользователя с id {id} не существует"
            ))),
        }
    }

    async fn by_email(&self, email: &String) -> AppResult<UserDAO> {
        match self.store.lock().await.values().find(|u| u.email == *email) {
            Some(user) => Ok(user.clone()),
            None => Err(AppError::NotFound(format!(
                "пользователя с почтой {email} не существует"
            ))),
        }
    }

    async fn save(&mut self, user: &UserDAO) -> AppResult<()> {
        let _ = self.store.lock().await.insert(user.id, user.clone());
        Ok(())
    }
}
