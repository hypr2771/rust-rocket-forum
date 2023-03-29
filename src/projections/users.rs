use crate::rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct User {
    id: String,
    email: String,
    password: String,
    logins: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Users {
    pub users: Vec<User>,
    size: usize
}

impl User {
    pub fn new(id: String, email: String, password: String) -> Self {
        Self {
            id,
            email,
            password,
            logins: 1,
        }
    }
}

impl Users {
    pub fn of(users: Vec<User>) -> Self {
        Self { size: users.len(), users }
    }

    pub fn append(&mut self, users: &mut Vec<User>){
        self.users.append(users);
        self.size = self.users.len();
    }

    pub fn push(&mut self, user: User){
        self.users.push(user);
        self.size += 1;
    }
}
