use std::fs::{OpenOptions, File};
use std::path::PathBuf;
use std::process::{Command, Stdio, ExitStatus};

pub struct Executor {
    log_file: PathBuf
}

impl Executor {
    pub fn new(log_file: PathBuf) -> Self {
        Self {
            log_file
        }
    }

    pub fn get_file(&self) -> File {
        OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&self.log_file)
            .unwrap()
    }

    pub fn execute(&self , directory: PathBuf , command: Vec<&str>) -> ExitStatus {
        let stdout = Stdio::from(self.get_file());
        let stderr = Stdio::from(self.get_file());

        Command::new("faketty")
            .current_dir(directory)
            .stdout(stdout)
            .stderr(stderr)
            .args(command)
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
    }
}

