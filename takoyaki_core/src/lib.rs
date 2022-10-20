pub mod plugin;
pub mod takoyaki;
pub mod ready_state;

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use crate::{plugin::Plugin, ready_state::ReadyState};

    use super::*;

    #[tokio::test]
    async fn main_test() {
        #[derive(Deserialize , Default , Debug)]
        pub struct Sample {
            id: u64
        }

        pub struct GitHubSamplePlugin {

        }

        impl<'a> Plugin<'a , Sample> for GitHubSamplePlugin {
            fn new() -> Self {
                Self {

                }
            }

            fn name(&self) -> &'a str {
                return "github"
            } 

            fn ready(&self) -> ReadyState {
                ReadyState::from_reqwest(reqwest::Client::new().get("https://jsonplaceholder.typicode.com/todos/1"))
            }

            fn execute(&self , data: Sample) {
                println!("{:?}" , data);
            }
        }

        let mut takoyaki = takoyaki::Takoyaki::<'static , Sample>::new();

        takoyaki.plug(Box::new(GitHubSamplePlugin::new()));


        println!("{}" , "HAHAHAH");

        takoyaki.start().await;
    }
}
