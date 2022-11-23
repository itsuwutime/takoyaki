use rocket::{request::{FromRequest , Request, Outcome}, http::Status};
use std::vec::IntoIter;

#[derive(Default, Debug)]
pub struct AuthGuard {

}

#[derive(Debug)]
pub enum Error {
    InvalidAuthorizationHeader,
    InvalidToken
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self , Self::Error> {
        let authorize_header: Vec<&str> = request.headers().get_one("Authorization").unwrap_or("").split_whitespace().collect();

        // Get the raw header
        let mut raw_token = authorize_header.iter();

        // Check if the raw header starts with a `Bearer`
        if raw_token.nth(0).unwrap_or(&"").to_lowercase() != "bearer" {
            return Outcome::Failure((Status::Unauthorized , Error::InvalidAuthorizationHeader))
        }

        Outcome::Success(Self::default())
    }
}
