use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, DateTime},
    error::Error,
    options::{ClientOptions, Credential, ServerAddress},
    Client, Collection,
};

use crate::projections::users::{Role, User};

pub struct UserRepository {
    collection: Collection<User>,
}

impl UserRepository {
    pub fn init() -> Self {
        let address = ServerAddress::Tcp {
            host: String::from("localhost"),
            port: Some(27017),
        };
        let credentials = Credential::builder()
            .username(String::from("root"))
            .password(String::from("example"))
            .build();
        let options = ClientOptions::builder()
            .hosts(vec![address])
            .credential(credentials)
            .build();

        let client = Client::with_options(options).unwrap();
        let database = client.database("rocket");
        let collection = database.collection("users");

        UserRepository { collection }
    }

    pub async fn get(&self) -> Result<Vec<User>, Error> {
        let users = self.collection.find(None, None).await?;

        let collected: Vec<Result<User, _>> = users.collect().await;
        Ok(collected.into_iter().map(|user| user.unwrap()).collect())
    }

    pub async fn _get_one(&self, id: ObjectId) -> Result<Option<User>, Error> {
        self.collection.find_one(Some(doc! {"_id": id}), None).await
    }

    pub async fn get_one_by_mail(&self, email: String) -> Result<Option<User>, Error> {
        self.collection
            .find_one(Some(doc! {"email": email}), None)
            .await
    }

    pub async fn put(&self, user: User) -> Result<User, Error> {
        let created = User {
            _id: Some(ObjectId::new()),
            creation: Some(DateTime::now()),
            requests: Some(1),
            role: Some(Role::Khey),
            ..user
        };

        self.collection
            .insert_one(created.clone(), None)
            .await
            .map(|_| created)
    }
}
