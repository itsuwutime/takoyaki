use super::Command;
use colored::*;
use crate::LOGGER;
use anyhow::Result;
use std::collections::HashMap;
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ErrorResponse {

}

#[derive(Deserialize)]
pub struct AuthResponse {
    error: Option<ErrorResponse>
}

pub struct Authenticate {

}

#[async_trait]
impl Command for Authenticate {
    fn new() -> Self {
        Self {

        }
    }

    fn prefix(&self) -> &str {
        "/auth"
    }

    async fn respond(&self , args: Vec<&str>) -> Result<&str> {
        // Check if the args is atleast 1 length long
        if args.len() < 1 {
            return Ok("Must provide atleast one argument for the token")
        }

        // Create a json body with the token
        let mut body = HashMap::new();

        // Add the token
        body.insert("idToken" , args[0]);

        // Create a new client
        let resp: AuthResponse = reqwest::Client::new()
            .post("https://identitytoolkit.googleapis.com/v1/accounts:lookup?key=AIzaSyDV8RkplTsJa9NXueaUUunH7_OjxfIydEc")
            .header("Content-Type" , "application/json")
            .json(&body)
            .send()
            .await?
            .json()
            .await?
        ;

        if resp.error.is_some() {
            LOGGER.error("Rejected client authentication (invalid_password)");

            Ok("Invalid passphrase!")
        } else {
            LOGGER.render("Successfully authorized a client (valid_password)".blue());

            Ok("Successfully authorized!")
        }
    }
}
