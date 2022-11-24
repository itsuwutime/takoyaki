// Import all the required dependencies
use reqwest::Result;
use std::collections::HashMap;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};
use serde::{Deserialize, Serialize};

// New Auth Guard sturct that acts as a auth middleware
#[derive(Default, Debug)]
pub struct AuthGuard {
    pub username: String,
}

// All the possible errors
#[derive(Debug)]
pub enum Error {
    InvalidAuthorizationHeader,
    InvalidToken,
}

// User struct
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    error: Option<ErrorResponse>,
    users: Option<Vec<UserType>>,
}

// The user type
#[derive(Serialize, Deserialize, Debug)]
pub struct UserType {
    #[serde(rename = "screenName")]
    screen_name: String,
}

// Empty struct
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {}

// Function to get the user from the token
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

// Impl FromRequest for AuthGuard
#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
    // Set the type of error that it handles
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Get the authorization header
        let authorize_header: Vec<&str> = request
            .headers()
            .get_one("Authorization")
            .unwrap_or("")
            .split_whitespace()
            .collect();

        // Get the raw header
        let mut raw_token = authorize_header.iter();

        // Check if the raw header starts with a `Bearer`
        if raw_token.next().unwrap_or(&"").to_lowercase() != "bearer" {
            return Outcome::Failure((Status::Unauthorized, Error::InvalidAuthorizationHeader));
        }

        // Try to get the current user
        let user = get_user(raw_token.next().unwrap_or(&"")).await.unwrap();

        // Check if there is some error
        if user.error.is_some() {
            return Outcome::Failure((Status::Unauthorized, Error::InvalidToken));
        }

        // Else return success ^-^
        Outcome::Success(Self {
            username: user.users.unwrap().into_iter().next().unwrap().screen_name,
        })
    }
}
