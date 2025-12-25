use crate::application::error::AppResult;

#[async_trait::async_trait]
pub trait KeyValueStorage {
    async fn get(&self, key: String) -> AppResult<String>;
    async fn set(&mut self, key: String, value: String) -> AppResult<()>;
    async fn remove(&mut self, key: String) -> AppResult<()>;
}
