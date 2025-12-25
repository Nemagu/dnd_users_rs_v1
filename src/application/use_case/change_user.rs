use std::sync::Arc;

use crate::application::{
    dto::ChangeUserCommand,
    error::{AppError, AppResult},
    model::User,
    repository::UserRepository,
    validator::{EmailValidator, PasswordHasher, PasswordValidator},
};

#[derive(Debug, Clone)]
pub struct ChangeUserUseCase<R, E, P, H>
where
    R: UserRepository,
    E: EmailValidator,
    P: PasswordValidator,
    H: PasswordHasher,
{
    repo: Arc<R>,
    email_validator: Arc<E>,
    password_validator: Arc<P>,
    password_hasher: Arc<H>,
}

impl<R, E, P, H> ChangeUserUseCase<R, E, P, H>
where
    R: UserRepository,
    E: EmailValidator,
    P: PasswordValidator,
    H: PasswordHasher,
{
    pub fn new(
        repo: Arc<R>,
        email_validator: Arc<E>,
        password_validator: Arc<P>,
        password_hasher: Arc<H>,
    ) -> Self {
        Self {
            repo,
            email_validator,
            password_validator,
            password_hasher,
        }
    }

    pub async fn execute(&self, command: ChangeUserCommand) -> AppResult<()> {
        let initiator: User = self.repo.by_id(&command.initiator_id).await?.try_into()?;
        if !initiator.can_edit_others() {
            return Err(AppError::NotAllowed);
        }
        let mut user: User;
        if command.initiator_id == command.user_id {
            user = initiator;
        } else {
            user = self.repo.by_id(&command.user_id).await?.try_into()?;
        }
        if let Some(email) = &command.email {
            self.email_validator.validate(email)?;
            user.new_email(email.to_owned())?;
        }
        if let Some(password) = &command.password {
            self.password_validator.validate(password, &user.email())?;
            user.new_password_hash(self.password_hasher.hash(password).await?)?;
        }
        if let Some(state) = &command.state {
            user.new_state(state.try_into()?)?;
        }
        if let Some(status) = &command.status {
            user.new_status(status.try_into()?)?;
        }
        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn test_change_user() {}
// }
