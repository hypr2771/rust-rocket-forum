use base64::{engine, Engine};
use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome, Request},
};

use crate::{projections::users::User, repositories::user_repository::UserRepository};

#[derive(Clone)]
pub struct Authorization {
    pub user: User,
}

#[derive(Debug, Clone, Copy)]
pub enum AuthorizationError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Authorization {
    type Error = AuthorizationError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match req.headers().get_one("authorization") {
            None => Outcome::Failure((Status::Forbidden, AuthorizationError::Missing)),
            Some(authorization) => {
                let result = req
                    .local_cache_async(async {
                        match req.rocket().state::<UserRepository>() {
                            Some(repository) => {
                                let decoded_result = engine::general_purpose::STANDARD
                                    .decode(authorization.replace("Basic ", ""))
                                    .map(|decoded| String::from_utf8(decoded));
                                match decoded_result {
                                    Ok(Ok(decoded)) => {
                                        println!("{}", decoded);

                                        // Major flaw today: mails are used as login but those can contains : character. If they do, this will fail
                                        if let Some((mail, password)) = decoded.split_once(":") {
                                            let user = repository
                                                .get_one_by_mail(String::from(mail))
                                                .await;

                                            match user {
                                                Ok(Some(user)) if user.password == password => {
                                                    Outcome::Success(Authorization { user })
                                                }
                                                _ => Outcome::Failure((
                                                    Status::InternalServerError,
                                                    AuthorizationError::Invalid,
                                                )),
                                            }
                                        } else {
                                            Outcome::Failure((
                                                Status::InternalServerError,
                                                AuthorizationError::Invalid,
                                            ))
                                        }
                                    }
                                    _ => Outcome::Failure((
                                        Status::InternalServerError,
                                        AuthorizationError::Invalid,
                                    )),
                                }
                            }
                            None => Outcome::Failure((
                                Status::InternalServerError,
                                AuthorizationError::Invalid,
                            )),
                        }
                    })
                    .await;

                match result {
                    Outcome::Success(a) => Outcome::Success(a.clone()),
                    Outcome::Failure(f) => Outcome::Failure(*f),
                    Outcome::Forward(fw) => Outcome::Forward(*fw),
                }
            }
        }
    }
}
