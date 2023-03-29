use futures::stream::{StreamExt};
use mongodb::{
    error::Error,
    options::{ClientOptions, ServerAddress, Credential},
    Client, Collection,
};

use crate::projections::users::User;

pub struct UserRepository {
    collection: Collection<User>,
}

impl UserRepository {
    pub fn init() -> Self {
        let address = ServerAddress::Tcp {
            host: String::from("localhost"),
            port: Some(27017),
        };
        let credentials = Credential::builder().username(String::from("root")).password(String::from("example")).build();
        let options = ClientOptions::builder().hosts(vec![address]).credential(credentials).build();

        let client = Client::with_options(options).unwrap();
        let database = client.database("rocket");
        let collection = database.collection("users");

        UserRepository { collection }
    }

    pub async fn get(&self) -> Result<Vec<User>, Error> {
        let result = self.collection.find(None, None).await;

        match result {
            Ok(cursor) => {
                let result: Vec<Result<User, _>> = cursor.collect().await;
                Ok(result.into_iter().map(|user| user.unwrap()).collect())
            }
            Err(error) => Err(error),
        }
    }

    pub async fn put(&self, user: User) -> Result<User, Error> {
        self.collection.insert_one(user.clone(), None).await.map(| _ | user)
    }
}
