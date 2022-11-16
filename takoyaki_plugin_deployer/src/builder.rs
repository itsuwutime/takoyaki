use std::process::{Stdio, Command};
use std::{thread, time::Duration};
use std::fs::File;

use crate::logger::Logger;

pub async fn create_new_deployment(uuid: String , github_url: String , branch: String , path: String , name: String) {
    let logger = Logger::new();
    let out = File::create("out.txt").unwrap();
    let out2 = File::create("out.txt").unwrap();

    let stdout = Stdio::from(out);
    let stdout2 = Stdio::from(out2);

    let command = Command::new("faketty")
        .args(["git" , "clone" , "https://www.github.com/rust-lang/rust"])
        .stdout(stdout)
        .stderr(stdout2)
        .spawn()
        .unwrap()
        .wait()
    ;

    logger.access(&format!("Deployment with uuid of {} was successful" , uuid));
}

