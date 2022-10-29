// Gonna add comments so that I don't end up forgetting in the future

// Make every module public
pub mod cache;
pub mod config;
pub mod helpers;
pub mod logger;
pub mod plugin;
pub mod path;
pub mod printable_grid;
pub mod ready_state;
pub mod takoyaki;

// Making reqwest public just in case someone needs to use it
pub use reqwest;

// Not so good tests till now, but I would add them in near future. I would be glad if someone adds it :)
#[cfg(test)]
mod tests {

}
