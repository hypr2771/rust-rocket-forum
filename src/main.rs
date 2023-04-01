#[macro_use]
extern crate rocket;

use projections::users::Users;
use repositories::topic_repository::TopicRepository;
use repositories::user_repository::UserRepository;
use rocket::serde::json::Json;
use rocket::State;

use crate::projections::topics::{Topic, Topics};
//use crate::guards::api_key::ApiKey;
use crate::projections::users::User;

mod guards;
mod projections;
mod repositories;

#[get("/users", format = "json")]
async fn get_users(repository: &State<UserRepository>) -> Json<Users> {
    let users = repository.get().await;
    Json(Users::of(users.unwrap()))
}

#[put("/users", format = "json", data = "<user>")]
async fn create_user(user: Json<User>, repository: &State<UserRepository>) -> Json<User> {
    let inner = user.into_inner();
    Json(repository.put(inner).await.unwrap())
}

#[get("/topics", format = "json")]
async fn get_topics(repository: &State<TopicRepository>) -> Json<Topics> {
    let topics = repository.get().await;
    Json(Topics::of(topics.unwrap()))
}

#[put("/topics", format = "json", data = "<topic>")]
async fn create_topic(topic: Json<Topic>, topic_repository: &State<TopicRepository>, user_repository: &State<UserRepository>) -> Json<Topic> {
    let inner = topic.into_inner();

    match user_repository.get_one(inner.author).await {
        Ok(Some(_)) => Json(topic_repository.put(inner).await.unwrap()),
        Ok(None) => panic!(),
        Err(_) => panic!()
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
