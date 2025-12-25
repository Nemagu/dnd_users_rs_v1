use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    InvalidData(String),
    #[error("пользователь не активен")]
    NotActive,
    #[error("вы не можете совершить это действие")]
    NotAllowed,
    #[error("{0}")]
    Internal(String),
}

pub type AppResult<T> = Result<T, AppError>;
