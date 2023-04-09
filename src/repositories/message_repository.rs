use futures::stream::StreamExt;

use mongodb::{
    bson::{doc, oid::ObjectId, DateTime},
    error::Error,
    options::{ClientOptions, Credential, ServerAddress},
    Client, Collection,
};

use crate::projections::topics::Message;

pub struct MessageRepository {
    pub collection: Collection<Message>,
}

impl MessageRepository {
    pub fn init() -> Self {
        let client = Client::with_options(
            ClientOptions::builder()
                .hosts(vec![ServerAddress::Tcp {
                    host: String::from("localhost"),
                    port: Some(27017),
                }])
                .credential(Some(
                    Credential::builder()
                        .username(String::from("root"))
                        .password(String::from("example"))
                        .build(),
                ))
                .build(),
        );

        let database = client.ok().unwrap().database("rocket");
        let collection = database.collection("messages");

        Self { collection }
    }

    pub async fn get(&self, topic: ObjectId) -> Result<Vec<Message>, Error> {
        let messages = self
            .collection
            .find(Some(doc! {"topic": topic}), None)
            .await?;

        let collected: Vec<Result<Message, Error>> = messages.collect().await;

        Ok(collected
            .into_iter()
            .map(|message| message.unwrap())
            .collect())
    }

    pub async fn put(&self, message: Message) -> Result<Message, Error> {
        let created = Message {
            _id: Some(ObjectId::new()),
            date: Some(DateTime::now()),
            ..message.clone()
        };

        self.collection
            .insert_one(created.clone(), None)
            .await
            .map(|_| created)
    }
}
