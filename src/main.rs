#[macro_use]
extern crate rocket;

use mongodb::bson::oid::ObjectId;
use projections::topics::Message;
use projections::users::Users;
use repositories::message_repository::MessageRepository;
use repositories::topic_repository::TopicRepository;
use repositories::user_repository::UserRepository;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;

use crate::guards::api_key::ApiKey;
use crate::guards::authorization::Authorization;
use crate::projections::api_errors::ApiError;
use crate::projections::topics::{PagedTopic, Topic, Topics};
//use crate::guards::api_key::ApiKey;
use crate::projections::users::User;

mod guards;
mod projections;
mod repositories;

#[get("/users", format = "json")]
async fn get_users(
    repository: &State<UserRepository>,
    _api_key: ApiKey,
) -> Result<Custom<Json<Users>>, ApiError> {
    let result = repository.get().await;

    match result {
        Ok(users) => Ok(Custom(Status::Ok, Json(Users::of(users)))),
        Err(error) => {
            println!("Received an error when trying to get all users {}", error);
            Err(ApiError::UsersNotResolved(format!("{}", error)))
        }
    }
}

#[put("/users", format = "json", data = "<user>")]
async fn create_user(
    user: Json<User>,
    repository: &State<UserRepository>,
) -> Result<Custom<Json<User>>, ApiError> {
    let inner = user.clone().into_inner();
    let result = repository.put(inner).await;

    match result {
        Ok(created) => Ok(Custom(Status::Created, Json(created))),
        Err(error) => {
            println!(
                "Received an error when trying to create user {:?}: {}",
                user, error
            );
            Err(ApiError::UserNotCreated(format!("{}", error)))
        }
    }
}

#[get("/topics", format = "json")]
async fn get_topics(repository: &State<TopicRepository>) -> Result<Custom<Json<Topics>>, ApiError> {
    let result = repository.get().await;

    match result {
        Ok(topics) => Ok(Custom(Status::Ok, Json(Topics::of(topics)))),
        Err(error) => {
            println!("Received an error when trying to get all topics {}", error);
            Err(ApiError::TopicsNotResolved(format!("{}", error)))
        }
    }
}

#[put("/topics", format = "json", data = "<topic>")]
async fn create_topic(
    topic: Json<Topic>,
    topic_repository: &State<TopicRepository>,
    authorization: Authorization,
) -> Result<Custom<Json<Topic>>, ApiError> {
    let inner = topic.into_inner();

    // Can unwrap because Authorization guard passed => we necessarily have a user
    let author = authorization.user._id;

    // Override author with authorized one
    let created = topic_repository.put(Topic { author, ..inner }).await;

    match created {
        Ok(topic) => Ok(Custom(Status::Created, Json(topic))),
        Err(error) => {
            println!("Received an error when trying to create topic {}", error);
            Err(ApiError::TopicNotCreated(format!("{}", error)))
        }
    }
}

#[get("/topics/<topic>")]
async fn get_topic(
    topic: &str,
    topic_repository: &State<TopicRepository>,
    message_repository: &State<MessageRepository>,
) -> Result<Custom<Json<PagedTopic>>, ApiError> {
    let topic_id = ObjectId::parse_str(topic);

    let result = topic_repository.get_one(topic_id.ok().unwrap()).await;

    match result {
        Ok(Some(topic)) => {
            let messages = message_repository.get(topic._id.unwrap()).await;

            let messages = match messages {
                Ok(messages) => messages,
                Err(_) => vec![],
            };

            Ok(Custom(Status::Ok, Json(topic.paged(messages))))
        }
        Ok(None) => {
            println!("Topic {} does not exists", topic);
            Err(ApiError::TopicNotCreated(format!(
                "Topic {} does not exists",
                topic
            )))
        }
        Err(error) => {
            println!("Received an error when trying to get all topics {}", error);
            Err(ApiError::TopicsNotResolved(format!("{}", error)))
        }
    }
}

#[put("/topics/<topic>/messages", format = "json", data = "<message>")]
async fn create_message(
    topic: String,
    message: Json<Message>,
    topic_repository: &State<TopicRepository>,
    message_repository: &State<MessageRepository>,
    authorization: Authorization,
) -> Result<Custom<Json<Message>>, ApiError> {
    let resolved_topic = topic_repository
        .get_one(ObjectId::parse_str(topic.clone()).unwrap())
        .await;

    match resolved_topic {
        Ok(Some(resolved_topic)) => {
            let created = message_repository
                .put(Message {
                    author: authorization.user._id,
                    topic: resolved_topic._id,
                    ..message.into_inner()
                })
                .await;

            match created {
                Ok(created) => Ok(Custom(Status::Ok, Json(created))),
                Err(error) => {
                    println!("Received an error when creating message {}", error);
                    Err(ApiError::MessageNotCreated(format!("{}", error)))
                }
            }
        }
        Ok(None) => {
            println!("Topic {} does not exists", topic);
            Err(ApiError::TopicsNotResolved(format!(
                "Topic {} does not exists",
                topic
            )))
        }
        Err(error) => {
            println!("Received an error when creating message {}", error);
            Err(ApiError::MessageNotCreated(format!("{}", error)))
        }
    }
}

#[launch]
async fn rocket() -> _ {
    let user_repository = UserRepository::init();
    let topic_repository = TopicRepository::init();
    let message_repository = MessageRepository::init();

    rocket::build()
        .manage(user_repository)
        .manage(topic_repository)
        .manage(message_repository)
        .mount(
            "/",
            routes![get_users, create_user, get_topics, create_topic, get_topic, create_message],
        )
}
