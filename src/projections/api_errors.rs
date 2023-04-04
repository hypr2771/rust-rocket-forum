use crate::rocket::serde::{Deserialize, Serialize};

#[derive(Responder, Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub enum ApiError {
    #[response(status = 412)]
    UserNotFound(String),
    #[response(status = 500)]
    UsersNotResolved(String),
    #[response(status = 500)]
    UserNotCreated(String),
    #[response(status = 500)]
    TopicNotCreated(String),
    #[response(status = 500)]
    TopicsNotResolved(String),
    #[response(status = 500)]
    MessageNotCreated(String),
}
