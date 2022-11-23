use std::collections::HashMap;
use reqwest::Result;

use rocket::{request::{FromRequest , Request, Outcome}, http::Status};
use serde::{Serialize, Deserialize};

#[derive(Serialize , Deserialize, Debug)]
pub struct User {
    error: Option<ErrorResponse>,
    users: Option<Vec<UserType>>
}

#[derive(Serialize , Deserialize, Debug)]
pub struct UserType {
    #[serde(rename = "screenName")]
    screen_name: String
}

#[derive(Serialize , Deserialize, Debug)]
pub struct ErrorResponse {

}

async fn get_user(token: &str) -> Result<User> {
    // Create a new client
    let client = reqwest::Client::new();


    // Create a new empty body
    let mut body = HashMap::new();

    // Set the token
    body.insert("idToken", token);

    // Create a post request
    let resp: User = client
        .post("https://identitytoolkit.googleapis.com/v1/accounts:lookup?key=AIzaSyDV8RkplTsJa9NXueaUUunH7_OjxfIydEc")
        .header("Content-Type" , "application/json")
        .json(&body)
        .send()
        .await?
        .json()
        .await?
    ;

    Ok(resp)
}

#[derive(Default, Debug)]
pub struct AuthGuard {
    pub username: String
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
        if raw_token.next().unwrap_or(&"").to_lowercase() != "bearer" {
            return Outcome::Failure((Status::Unauthorized , Error::InvalidAuthorizationHeader))
        }

        // Try to get the current user
        let user = get_user(raw_token.next().unwrap_or(&"")).await.unwrap();

        if user.error.is_some() {
            return Outcome::Failure((Status::Unauthorized, Error::InvalidToken))
        }

        Outcome::Success(Self {
            username: user.users.unwrap().into_iter().next().unwrap().screen_name
        })
    }
}
