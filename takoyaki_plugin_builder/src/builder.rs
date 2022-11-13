use std::process::Command;

use crate::logger::Logger;
use colored::*;

pub fn create_new_build(github_url: &str , branch: &str , path: &str , uuid: &str) {
    // Create a new logger class
    let logger = Logger::new();

    // Show some success message
    logger.success(&format!("Cloning github repository - {} (branch: {})" , github_url , branch.bold()));

    // Spawn git command
    Command::new("git")
        .args(["clone" , "--branch" , branch , github_url , ".temp"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
    ;

    // Success message :)
    logger.success("Successfully cloned the repository!");

    // Build for production
    logger.success("Building for production - Running `cargo build --release`");

    Command::new("/bin/bash")
        .arg("-c")
        .arg("cd .temp && cargo build --release")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
    ;

    // Successful build
    logger.success("Successfully build the plugin for production!");

    // Publishing...
    logger.success("Publishing plugin...");

    // Command::new("mv")
    //     .args([format!("./.temp/target/release/")])
}
