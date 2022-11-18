use std::{fs::{File, OpenOptions}, path::PathBuf, process::{Stdio, Command}};

pub fn execute(out: PathBuf , args: Vec<&str> , current_dir: &PathBuf) {
    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(out)
        .unwrap();

    // Get standard output for the log file
    let stdout = Stdio::from(log_file.try_clone().unwrap());
    let stderr = Stdio::from(log_file);

    // Create git clone command
    let command = Command::new("faketty")
        .current_dir(current_dir)
        .args(args)
        .stdout(stdout)
        .stderr(stderr)
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
    ;
}