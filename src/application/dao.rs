use uuid::Uuid;

use crate::{
    application::error::AppError,
    application::model::{State, Status, User},
};

#[derive(Debug, Clone)]
pub struct UserDAO {
    pub id: Uuid,
    pub email: String,
    pub state: String,
    pub status: String,
    pub password_hash: String,
    pub version: u64,
}

impl TryFrom<&User> for UserDAO {
    type Error = AppError;
    fn try_from(value: &User) -> Result<Self, Self::Error> {
        Ok(UserDAO {
            id: value.id(),
            email: value.email(),
            state: value.state().to_string(),
            status: value.status().to_string(),
            password_hash: value.password_hash(),
            version: value.current_version(),
        })
    }
}

impl From<User> for UserDAO {
    fn from(value: User) -> Self {
        Self {
            id: value.id(),
            email: value.email(),
            state: value.state().to_string(),
            status: value.status().to_string(),
            password_hash: value.password_hash(),
            version: value.current_version(),
        }
    }
}

impl TryInto<User> for UserDAO {
    type Error = AppError;
    fn try_into(self) -> Result<User, Self::Error> {
        let state: State = self.state.try_into()?;
        let status: Status = self.status.try_into()?;
        Ok(User::restore(
            self.id,
            self.email,
            state,
            status,
            self.password_hash,
            self.version,
        )?)
    }
}
