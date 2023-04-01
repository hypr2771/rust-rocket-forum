#[macro_use]
extern crate rocket;

use projections::users::Users;
use repositories::topic_repository::TopicRepository;
use repositories::user_repository::UserRepository;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;

use crate::guards::api_key::ApiKey;
use crate::guards::authorization::Authorization;
use crate::projections::api_errors::ApiError;
use crate::projections::topics::{Topic, Topics};
//use crate::guards::api_key::ApiKey;
use crate::projections::users::User;

mod guards;
mod projections;
mod repositories;

#[get("/users", format = "json")]
async fn get_users(repository: &State<UserRepository>, _authorization: Authorization, _api_key: ApiKey) -> Result<Custom<Json<Users>>, ApiError> {
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
    user_repository: &State<UserRepository>,
) -> Result<Custom<Json<Topic>>, ApiError> {
    let inner = topic.into_inner();

    match user_repository.get_one(inner.author).await {
        Ok(Some(_)) => {
            let created = topic_repository.put(inner.clone()).await;

            match created {
                Ok(topic) => Ok(Custom(Status::Created, Json(topic))),
                Err(error) => {
                    println!("Received an error when trying to create topic {}", error);
                    Err(ApiError::TopicNotCreated(format!("{}", error)))
                }
            }
        }
        Ok(None) => {
            println!("User {} does not exists", inner.author);
            Err(ApiError::UserNotFound(format!(
                "User {} does not exists",
                inner.author
            )))
        }
        Err(error) => {
            println!("Received an error when trying to get all users {}", error);
            Err(ApiError::UsersNotResolved(format!("{}", error)))
        },
    }
}

#[launch]
async fn rocket() -> _ {
    let user_repository = UserRepository::init();
    let topic_repository = TopicRepository::init();

    rocket::build()
        .manage(user_repository)
        .manage(topic_repository)
        .mount(
            "/",
            routes![get_users, create_user, get_topics, create_topic],
        )
}
