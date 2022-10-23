pub mod config;
pub mod cache;
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
}

