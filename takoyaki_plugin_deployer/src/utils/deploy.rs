use crate::utils::Setup;
use std::{fs::{File, create_dir_all}, process::{Stdio, Command}, path::PathBuf};

pub fn execute(out_file: File, command: &Vec<&str>, cwd: &PathBuf) {
    let stdout = Stdio::from(out_file.try_clone().unwrap());
    let stderr = Stdio::from(out_file);

    Command::new("faketty")
        .stdout(stdout)
        .stderr(stderr)
        .current_dir(cwd)
        .args(command)
        .spawn().unwrap()
        .wait().unwrap()
    ;
}

pub async fn create_deployment(uuid: String, name: String, github_url: String, branch: String, path: String) {
    // Get a new setup instance
    let setup = Setup::new();

    // Get the binary path
    let binary_path = PathBuf::from("target").join("release").join(&name);

    // Vector of commands that is going to be ran
    let commands = vec![
        vec!["git" , "clone" , "-b" , &branch , &github_url , &name], // Clone the github repository
        vec!["cargo" , "build" , "--release"], // Build for production
        vec!["mv" , binary_path.to_str().unwrap() , setup.plugins_dir.to_str().unwrap()], // Build for production
    ];

    // Vector of paths that are going to be changed to according to the command priority
    let directories = vec![
        setup.build_dir.clone(),
        setup.build_dir.join(&name).join(&path),
        setup.build_dir.join(&name).join(&path)
    ];

    // Create deployments dir
    create_dir_all(setup.deployments_dir.clone().join(&name)).unwrap();

    // Get stdout and stderr
    let out_file = File::create(setup.deployments_dir.clone().join(&name).join(format!("{}.txt" , uuid))).unwrap();

    for (command, cwd) in commands.iter().zip(directories.iter()) {
        execute(
            out_file.try_clone().unwrap(),
            command, 
            cwd
        )
    }
}
