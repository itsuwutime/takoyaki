// Import dependencies
use crate::utils::Setup;
use std::{
    fs::{create_dir_all, File},
    path::PathBuf,
    io::Result,
    process::{Command, Stdio},
};

// Create a new execute function that executes and pushes the output to the file
pub fn execute(out_file: File, command: &Vec<&str>, cwd: &PathBuf) -> Result<()> {
    // Get the stdout and stderr
    let stdout = Stdio::from(out_file.try_clone().unwrap());
    let stderr = Stdio::from(out_file);

    // Create a new command and execute it
    Command::new("faketty")
        .stdout(stdout)
        .stderr(stderr)
        .current_dir(cwd)
        .args(command)
        .spawn()?
        .wait()?;

    // Ok!
    Ok(())
}

pub async fn create_deployment(
    username: String,
    uuid: String,
    name: String,
    github_url: String,
    branch: String,
    path: String,
) -> Result<()> {
    // Get a new setup instance
    let setup = Setup::instance();

    // Get the binary path
    let binary_path: PathBuf = ["target" , "release", &name].iter().collect();

    // Vector of commands that is going to be ran
    let commands = vec![
        vec!["git", "clone", "-b", &branch, &github_url, &name], // Clone the github repository
        vec!["cargo", "build", "--release"],                     // Build for production
        vec![
            "mv",
            binary_path.to_str().unwrap(),
            setup.plugins_dir.to_str().unwrap(),
        ], // Move the built plugin to the 
    ];

    // Vector of paths that are going to be changed to according to the command priority
    let directories = vec![
        setup.build_dir.clone().join(&username),
        setup.build_dir.join(&username).join(&name).join(&path),
        setup.build_dir.join(&username).join(&name).join(&path),
    ];

    // Create deployments dir
    create_dir_all(setup.deployments_dir.clone().join(&username).join(&name)).unwrap();
    create_dir_all(setup.build_dir.clone().join(&username)).unwrap();

    // Get stdout and stderr
    let out_file = File::create(
        setup
            .deployments_dir
            .clone()
            .join(&username)
            .join(&name)
            .join(format!("{}.txt", uuid)),
    )
    .unwrap();

    for (command, cwd) in commands.iter().zip(directories.iter()) {
        execute(out_file.try_clone().unwrap(), command, cwd)?
    }

    Ok(())
}
