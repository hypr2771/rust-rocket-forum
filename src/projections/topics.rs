use crate::rocket::serde::{Deserialize, Serialize};
use mongodb::bson::{oid::ObjectId, DateTime};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Topic {
    pub _id: Option<ObjectId>,
    pub title: String,
    pub author: Option<ObjectId>,
    pub date: Option<DateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Topics {
    pub topics: Vec<Topic>,
    size: usize,
}
impl Topics {
    pub fn of(topics: Vec<Topic>) -> Topics {
        let size = topics.len();

        Self { topics, size }
    }
}
