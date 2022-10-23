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
    

    #[derive(Deserialize , Default , Debug)]
    pub struct Sample {
        #[serde(rename = "id")]
        _id: u64
    }
}

