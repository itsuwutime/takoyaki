use std::{fs::{create_dir_all, File}, process::Stdio};

pub fn create_deployment(git_url: &str, branch: &str, path: &str, name: &str , uuid: &str) {
    // Get the takoyaki's root directory of the user ($HOME/.takoyaki)
    let takoyaki_root = dirs::home_dir().unwrap().join(".takoyaki");

    // Run the git clone command
    let deployments_dir = takoyaki_root.clone().join("deployments").join(name);

    // Create the directory just in case if it is not there 
    create_dir_all(&deployments_dir).unwrap();

    // Create a new deployment log file
    let log_file = File::create(deployments_dir.join(format!("{}.txt" , uuid))).unwrap();

    // Get standard output for the log file
    let stdout = Stdio::from(log_file);

    // Create git clone command

}
