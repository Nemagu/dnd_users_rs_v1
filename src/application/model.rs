use uuid::Uuid;

use crate::application::error::{AppError, AppResult};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Active,
    Frozen,
    Deleted,
}

impl TryFrom<String> for State {
    type Error = AppError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "active" => Ok(State::Active),
            "frozen" => Ok(State::Frozen),
            "deleted" => Ok(State::Deleted),
            _ => Err(AppError::InvalidData(format!(
                "состояния с названием {value} не существует"
            ))),
        }
    }
}

impl TryFrom<&String> for State {
    type Error = AppError;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "active" => Ok(State::Active),
            "frozen" => Ok(State::Frozen),
            "deleted" => Ok(State::Deleted),
            _ => Err(AppError::InvalidData(format!(
                "состояния с названием {value} не существует"
            ))),
        }
    }
}

impl TryFrom<&str> for State {
    type Error = AppError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "active" => Ok(State::Active),
            "frozen" => Ok(State::Frozen),
            "deleted" => Ok(State::Deleted),
            _ => Err(AppError::InvalidData(format!(
                "состояния с названием {value} не существует"
            ))),
        }
    }
}

impl ToString for State {
    fn to_string(&self) -> String {
        match self {
            Self::Active => "active".to_string(),
            Self::Frozen => "frozen".to_string(),
            Self::Deleted => "deleted".to_string(),
        }
    }
}

impl State {
    pub fn is_active(&self) -> bool {
        match self {
            State::Active => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    User,
    Admin,
}

impl TryFrom<String> for Status {
    type Error = AppError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "user" => Ok(Status::User),
            "admin" => Ok(Status::Admin),
            _ => Err(AppError::InvalidData(format!(
                "статуса с названием {value} не существует"
            ))),
        }
    }
}

impl TryFrom<&String> for Status {
    type Error = AppError;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "user" => Ok(Status::User),
            "admin" => Ok(Status::Admin),
            _ => Err(AppError::InvalidData(format!(
                "статуса с названием {value} не существует"
            ))),
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = AppError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "user" => Ok(Status::User),
            "admin" => Ok(Status::Admin),
            _ => Err(AppError::InvalidData(format!(
                "статуса с названием {value} не существует"
            ))),
        }
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Self::Admin => "admin".to_string(),
            Self::User => "user".to_string(),
        }
    }
}

impl Status {
    pub fn is_admin(&self) -> bool {
        match self {
            Status::Admin => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct User {
    id: Uuid,
    email: String,
    state: State,
    status: Status,
    password_hash: String,
    version: u64,
}

impl User {
    pub fn new(id: Uuid, email: String, password_hash: String) -> AppResult<User> {
        Ok(User {
            id: id,
            email: email,
            state: State::Active,
            status: Status::User,
            password_hash: password_hash,
            version: 0,
        })
    }

    pub fn restore(
        id: Uuid,
        email: String,
        state: State,
        status: Status,
        password_hash: String,
        version: u64,
    ) -> AppResult<User> {
        if version == 0 {
            return Err(AppError::Internal(
                "версия пользователя не может быть ровна 0".to_string(),
            ));
        }
        Ok(User {
            id: id,
            email: email,
            state: state,
            status: status,
            password_hash: password_hash,
            version: version,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn email(&self) -> String {
        self.email.clone()
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn password_hash(&self) -> String {
        self.password_hash.clone()
    }

    pub fn current_version(&self) -> u64 {
        self.version
    }

    pub fn modify_version(&self) -> u64 {
        self.version + 1
    }

    pub fn can_edit_others(&self) -> bool {
        self.state.is_active() && self.status.is_admin()
    }

    pub fn new_email(&mut self, email: String) -> AppResult<()> {
        self.check_state()?;
        if self.email == email {
            return Err(AppError::InvalidData(format!(
                "новая почта совпадает со старой"
            )));
        }
        self.email = email;
        Ok(())
    }

    pub fn new_state(&mut self, state: State) -> AppResult<()> {
        if self.state == state {
            return Err(AppError::InvalidData(
                "новое состояние совпадает со старым".to_string(),
            ));
        }
        self.state = state;
        Ok(())
    }

    pub fn new_status(&mut self, status: Status) -> AppResult<()> {
        self.check_state()?;
        if self.status == status {
            return Err(AppError::InvalidData(
                "новый статус совпадает со старым".to_string(),
            ));
        }
        self.status = status;
        Ok(())
    }

    pub fn new_password_hash(&mut self, password_hash: String) -> AppResult<()> {
        self.check_state()?;
        self.password_hash = password_hash;
        Ok(())
    }

    fn check_state(&self) -> AppResult<()> {
        if !self.state.is_active() {
            Err(AppError::NotActive)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn restore_user() -> User {
        User::restore(
            Uuid::new_v4(),
            "test@test.ru".to_string(),
            State::Active,
            Status::User,
            "123".to_string(),
            1,
        )
        .unwrap()
    }

    #[test]
    fn test_new_user_ok() {
        let user = User::new(
            Uuid::new_v4(),
            "test@test.ru".to_string(),
            "123".to_string(),
        );
        assert!(user.is_ok())
    }

    #[test]
    fn test_restore_user_ok() {
        match User::restore(
            Uuid::new_v4(),
            "test@test.ru".to_string(),
            State::Active,
            Status::User,
            "123".to_string(),
            1,
        ) {
            Ok(_) => (),
            Err(e) => {
                panic!("Expected error: {e}")
            }
        }
    }

    #[test]
    fn test_restore_user_version_is_0() {
        match User::restore(
            Uuid::new_v4(),
            "test@test.ru".to_string(),
            State::Active,
            Status::User,
            "123".to_string(),
            0,
        ) {
            Ok(_) => panic!("didn't get error"),
            Err(AppError::Internal(_)) => (),
            Err(e) => panic!("Expected error: {e}"),
        }
    }

    #[test]
    fn test_new_email_ok() {
        match restore_user().new_email("new_email@mail.com".to_string()) {
            Ok(_) => (),
            Err(e) => {
                panic!("Expected error: {e}")
            }
        }
    }

    #[test]
    fn test_new_email_same() {
        let mut user = restore_user();
        match user.new_email(user.email()) {
            Ok(_) => panic!("didn't get error"),
            Err(AppError::InvalidData(_)) => (),
            Err(e) => panic!("Expected error: {e}"),
        }
    }

    #[test]
    fn test_new_email_user_not_active() {
        let mut user = restore_user();
        user.state = State::Frozen;
        match user.new_email("new_email@mail.com".to_string()) {
            Ok(_) => panic!("didn't get error"),
            Err(AppError::NotActive) => (),
            Err(e) => panic!("Expected error: {e}"),
        }
    }

    #[test]
    fn test_new_password_hash_ok() {
        match restore_user().new_password_hash("123".to_string()) {
            Ok(_) => (),
            Err(e) => {
                panic!("Expected error: {e}")
            }
        }
    }

    #[test]
    fn test_new_password_hash_user_not_active() {
        let mut user = restore_user();
        user.state = State::Frozen;
        match user.new_password_hash("123".to_string()) {
            Ok(_) => panic!("didn't get error"),
            Err(AppError::NotActive) => (),
            Err(e) => panic!("Expected error: {e}"),
        }
    }

    #[test]
    fn test_new_state_ok() {
        match restore_user().new_state(State::Frozen) {
            Ok(_) => (),
            Err(e) => {
                panic!("Expected error: {e}")
            }
        }
    }

    #[test]
    fn test_new_state_same() {
        let mut user = restore_user();
        match user.new_state(user.state()) {
            Ok(_) => panic!("didn't get error"),
            Err(AppError::InvalidData(_)) => (),
            Err(e) => panic!("Expected error: {e}"),
        }
    }

    #[test]
    fn test_new_state_user_not_active() {
        let mut user = restore_user();
        user.state = State::Frozen;
        match user.new_state(State::Active) {
            Ok(_) => (),
            Err(e) => panic!("Expected error: {e}"),
        }
    }

    #[test]
    fn test_new_status_ok() {
        match restore_user().new_status(Status::Admin) {
            Ok(_) => (),
            Err(e) => {
                panic!("Expected error: {e}")
            }
        }
    }

    #[test]
    fn test_new_status_same() {
        let mut user = restore_user();
        match user.new_status(user.status()) {
            Ok(_) => panic!("didn't get error"),
            Err(AppError::InvalidData(_)) => (),
            Err(e) => panic!("Expected error: {e}"),
        }
    }

    #[test]
    fn test_new_status_user_not_active() {
        let mut user = restore_user();
        user.state = State::Frozen;
        match user.new_status(Status::Admin) {
            Ok(_) => panic!("didn't get error"),
            Err(AppError::NotActive) => (),
            Err(e) => panic!("Expected error: {e}"),
        }
    }
}
