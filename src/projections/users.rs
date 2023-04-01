use crate::rocket::serde::{Deserialize, Serialize};
use mongodb::bson::{oid::ObjectId, DateTime};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub _id: Option<ObjectId>,
    pub email: String,
    pub password: String,
    pub role: Option<Role>,
    pub creation: Option<DateTime>,
    pub requests: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Role {
    Guest,
    Khey,
    Modo
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Users {
    pub users: Vec<User>,
    size: usize
}

impl Users {
    pub fn of(users: Vec<User>) -> Self {
        Self { size: users.len(), users }
    }
}
