use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, DateTime},
    error::Error,
    options::{ClientOptions, Credential, ServerAddress},
    Client, Collection,
};

use crate::projections::topics::Topic;

pub struct TopicRepository {
    collection: Collection<Topic>,
}

impl TopicRepository {
    pub fn init() -> Self {
        let client = Client::with_options(
            ClientOptions::builder()
                .hosts(vec![ServerAddress::Tcp {
                    host: String::from("localhost"),
                    port: Some(27017),
                }])
                .credential(
                    Credential::builder()
                        .username(String::from("root"))
                        .password(String::from("example"))
                        .build(),
                )
                .build(),
        )
        .unwrap();

        let database = client.database("rocket");
        let collection: Collection<Topic> = database.collection("posts");

        Self { collection }
    }

    pub async fn get(&self) -> Result<Vec<Topic>, Error> {
        let topics = self.collection.find(None, None).await?;

        let collected: Vec<Result<Topic, Error>> = topics.collect().await;
        Ok(collected.into_iter().map(|topic| topic.unwrap()).collect())
    }

    pub async fn get_one(&self, id: ObjectId) -> Result<Option<Topic>, Error> {
        self.collection.find_one(doc! {"_id": id}, None).await
    }

    pub async fn put(&self, topic: Topic) -> Result<Topic, Error> {
        let created = Topic {
            _id: Some(ObjectId::new()),
            date: Some(DateTime::now()),
            ..topic
        };

        self.collection
            .insert_one(created.clone(), None)
            .await
            .map(|_| created)
    }
}
