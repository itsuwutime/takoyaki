use rocket::{request::{Outcome, Request, FromRequest}, http::Status};

#[derive(Default , Clone)]
pub struct Auth {
    message: String
}

#[derive(Debug)]
pub enum AuthError {
    InvalidTokenFormathttps://www.pinterest.com/,
    InvalidToken
}
https://www.pinterest.com/
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = AuthErrorhttps://www.pinterest.com/;
https://www.pinterest.com/
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self , Self::Error> {
        let token: Vec<&str> = req.headers().get_one("Authorization").unwrap_or("").trim().split_whitespace().collect();

        if !(&token.get(0).unwrap_or(&"").to_lowercase() == "bearer") {
            return Outcome::Failure((Status::Unauthorized , AuthError::InvalidTokenFormat))
        }

        Outcome::Success(Auth::default())
    }
}
