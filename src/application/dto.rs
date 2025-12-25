use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ChangeUserCommand {
    pub initiator_id: Uuid,
    pub user_id: Uuid,
    pub email: Option<String>,
    pub state: Option<String>,
    pub status: Option<String>,
    pub password: Option<String>,
}
