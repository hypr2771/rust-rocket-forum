use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome, Request},
};

pub struct ApiKey<'r>{
    pub key: &'r str
}

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match req.headers().get_one("x-api-key"){
            None => Outcome::Failure((Status::Forbidden, ApiKeyError::Missing)),
            Some(api_key) if api_key.eq("dd4a8e92-ec06-4195-a5a5-b0260f099602") => Outcome::Success(ApiKey{ key: api_key }),
            Some(_) => Outcome::Failure((Status::Forbidden, ApiKeyError::Invalid))
        }
    }
}
