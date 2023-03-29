#[macro_use] extern crate rocket;

use projections::users::Users;
use repositories::user_repository::UserRepository;
use rocket::State;
use rocket::serde::json::Json;

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
    println!("{:?}", inner);
    Json(repository.put(inner).await.unwrap())
}

#[launch]
async fn rocket() -> _ {
    let state = UserRepository::init();

    rocket::build().manage(state).mount("/", routes![get_users, create_user])
}