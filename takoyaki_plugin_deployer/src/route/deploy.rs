use rocket::serde::{json::Json, Serialize, Deserialize};
use either::Either;
use crate::middlewares::{AuthGuard, Error};
use crate::utils::create_deployment;

#[derive(Deserialize , Debug)]
#[serde(crate = "rocket::serde")]
pub struct DeployData {
    pub name: String,
    pub github_url: String,
    pub branch: String,
    pub path: String
}

// Successful response type
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SuccessResponse {
    pub deployment_uuid: String
}

// Error response
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResponse<'a> {
    pub help: &'a str,
    pub message: &'a str
}

// Binded response
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[serde(transparent)]
pub struct Response<'a> {
    #[serde(with = "either::serde_untagged")]
    inner: Either<SuccessResponse, ErrorResponse<'a>>
}

#[post("/deploy" , format="application/json" , data="<data>")]
pub fn create_new_deployment<'a>(data: Json<DeployData>, auth_guard: Result<AuthGuard , Error>) -> Json<Response<'a>> {
    if auth_guard.is_err() {
        return match auth_guard.unwrap_err() {
            Error::InvalidToken => {
                Json(Response {
                    inner: Either::Right(ErrorResponse {
                        help: "https://takoyaki.kyeboard.me/docs/deployer/errors#invalid_token",
                        message: "The authentication token was invalid!"
                    })
                })
            },
            Error::InvalidAuthorizationHeader => {
                Json(Response {
                    inner: Either::Right(ErrorResponse {
                        help: "https://takoyaki.kyeboard.me/docs/deployer/errors#invalid_authorization_header",
                        message: "The authentication token was invalid!"
                    })
                })
            }
        }
    };

    let uuid = uuid::Uuid::new_v4().to_string();

    tokio::spawn(
        create_deployment(
            uuid.clone(), 
            data.name.clone(),
            data.github_url.clone(), 
            data.branch.clone(), 
            data.path.clone()
        )
    );

    Json(Response {
        inner: Either::Left(SuccessResponse {
            deployment_uuid: uuid
        })
    })
}

