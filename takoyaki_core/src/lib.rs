// Gonna add comments so that I don't end up forgetting in the future

mod cache;
mod config;
mod helpers;
mod logger;
mod plugin;
mod path;
mod printable_grid;
mod ready_state;
mod takoyaki;

// Export everything from every module
pub use cache::*;
pub use config::*;
pub use helpers::*;
pub use logger::*;
pub use plugin::*;
pub use path::*;
pub use printable_grid::*;
pub use ready_state::*;
pub use takoyaki::*;

// Making reqwest public just in case someone needs to use it
pub use reqwest;

// Not so good tests till now, but I would add them in near future. I would be glad if someone adds it :)
#[cfg(test)]
mod tests {

}
