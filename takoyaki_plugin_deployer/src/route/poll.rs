use crate::middlewares::{AuthGuard, Error};
use crate::utils::Setup;
use either::Either;
use rocket::response::stream::ReaderStream;
use rocket::tokio::fs::File;
use serde::Serialize;

#[derive(Serialize, Responder)]
pub struct ErrorResponse<'a> {
    error: &'a str,
}

#[get("/poll/<project>/<id>")]
pub fn poll_logs<'a>(
    project: String,
    id: String,
    auth_guard: Result<AuthGuard, Error>,
) -> Either<ReaderStream![File], ErrorResponse<'a>> {
    let setup = Setup::new();

    // if auth_guard.is_err() {
    //     return match auth_guard.unwrap_err() {
    //         Error::InvalidToken => {
    //             Json(Response {
    //                 inner: Either::Right(ErrorResponse {
    //                     help: "https://takoyaki.kyeboard.me/docs/deployer/errors#invalid_token",
    //                     message: "The authentication token was invalid!"
    //                 })
    //             })
    //         },
    //         Error::InvalidAuthorizationHeader => {
    //             Json(Response {
    //                 inner: Either::Right(ErrorResponse {
    //                     help: "https://takoyaki.kyeboard.me/docs/deployer/errors#invalid_authorization_header",
    //                     message: "The authentication token was invalid!"
    //                 })
    //             })
    //         }
    //     }
    // };
    let username = auth_guard.unwrap();

    let file_path = setup
        .deployments_dir
        .join(username.username)
        .join(project)
        .join(format!("{}.txt", id));

    if file_path.exists() {
        Either::Left(ReaderStream! {
            yield File::open(file_path).await.unwrap()
        })
    } else {
        Either::Right(ErrorResponse {
            error: "Cannot find any deployment for the specified project",
        })
    }
}
