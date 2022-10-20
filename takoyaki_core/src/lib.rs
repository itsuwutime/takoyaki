pub mod plugin;
pub mod takoyaki;
pub mod ready_state;
pub mod printable_grid;
pub use reqwest;

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use crate::{plugin::Plugin, ready_state::ReadyState, takoyaki , printable_grid::PrintableGrid};

    #[derive(Deserialize , Default , Debug)]
    pub struct Sample {
        #[serde(rename = "id")]
        _id: u64
    }

    pub struct SamplePlugin {

    }

    impl<'a> Plugin<'a , Sample> for SamplePlugin {
        fn new() -> Self {
            Self {

            }
        }

        fn name(&self) -> &'a str {
            "sample_plugin"
        } 

        fn ready(&self) -> ReadyState {
            ReadyState::from_reqwest(reqwest::Client::new().get("https://jsonplaceholder.typicode.com/todos/1"))
        }

        fn execute(&self , _data: Sample) -> PrintableGrid {
            PrintableGrid::new()
        }
    }


    #[tokio::test]
    #[should_panic]
    async fn start_without_plug() {
        let takoyaki = takoyaki::Takoyaki::<'static , Sample>::new();

        takoyaki.start().await.unwrap(); // Would panic!
    } 

    #[tokio::test]
    async fn start_with_plug() {
        let mut takoyaki = takoyaki::Takoyaki::<Sample>::new();
        let plugin = SamplePlugin::new();

        takoyaki.plug(&plugin);

        takoyaki.start().await.unwrap(); // Would not panic =)
    }
}

