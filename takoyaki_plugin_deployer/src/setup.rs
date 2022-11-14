use std::path::PathBuf;
use rand::Rng;

use crate::error::Error;

type Result<T> = std::result::Result<T , Error>;

pub fn setup() -> Result<()> {
    // Get the home directory
    let home_dir = PathBuf::from(std::env::var("HOME").map_err(|_| Error::NoHomeDirectoryFound)?);

    // Print the directory to be used for storing binaries
    println!(
        "==> Storing binaries at {}", 
        home_dir.join(".takoyaki").join("bin").display()
    );

    // Print the directory to be used for storing cache
    println!(
        "==> Storing plugin cache (files) at {}", 
        home_dir.join(".takoyaki").join("cache").display()
    );

    // Print the directory to be used for storing build logs
    println!(
        "==> Storing build logs at {}", 
        home_dir.join(".takoyaki").join("logs").display()
    );

    // Setup up a salt for encrypted responses
    println!("==> Setting up a random salt");

    // Generate salt
    let salt = rand::thread_rng().gen_range(100..1000);

    // Set the salt as the env 
    std::env::set_var("TAKOYAKI_SALT", salt.to_string());

    Ok(())
}

