use rocket::response::stream::ReaderStream;
use rocket::tokio::fs::File;

#[get("/poll/<project>/<id>")]
pub fn poll_logs(project: String, id: String) -> ReaderStream![File] {
    ReaderStream! {
        yield File::open("/home/voidcupboard/.takoyaki/deployments/worldhellosdj/github/1895c5df-4f40-4328-9f17-797357e0cd72.txt").await.unwrap()
    }
}

