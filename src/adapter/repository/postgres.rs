// use std::sync::Arc;

// use sqlx::PgPool;
// use uuid::Uuid;

// use crate::application::{
//     error::{AppError, AppResult},
//     logger::Logger,
//     repository::UserRepository,
// };

// #[derive(Debug)]
// pub struct PostgresUserRepository<'a, L>
// where
//     L: Logger,
// {
//     pool: &'a PgPool,
//     logger: Arc<L>,
// }

// impl<'a, L> PostgresUserRepository<'a, L>
// where
//     L: Logger,
// {
//     pub fn new(pool: &'a PgPool, logger: Arc<L>) -> Self {
//         Self {
//             pool: pool,
//             logger: logger,
//         }
//     }
// }

// #[async_trait::async_trait]
// impl<'a, L> UserRepository for PostgresUserRepository<'a, L>
// where
//     L: Logger,
// {
//     async fn next_id(&self) -> AppResult<Uuid> {
//         Ok(Uuid::new_v4())
//     }

//     async fn id_exists(&self, id: &Uuid) -> AppResult<bool> {
//         let rec = sqlx::query!(
//             "SELECT EXISTS(SELECT 1 FROM users WHERE id = $1)",
//             id.to_string()
//         )
//         .fetch_one(self.pool)
//         .await;
//         match rec {
//             Ok(rec) => Ok(rec.exists),
//             Err(err) => {
//                 self.logger
//                     .error(&("error checking if id exists: " + err.to_string()));
//                 Err(AppError::Internal(err.to_string()))
//             }
//         }
//     }
//     async fn email_exists(&self, email: &String) -> AppResult<bool>;
//     async fn by_id(&self, id: &Uuid) -> AppResult<UserDAO>;
//     async fn by_email(&self, email: &String) -> AppResult<UserDAO>;
//     async fn save(&mut self, user: &UserDAO) -> AppResult<()>;
// }
