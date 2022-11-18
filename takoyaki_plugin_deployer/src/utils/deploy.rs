use std::{fs::{create_dir_all, File}, process::{Stdio, Command}};

use crate::execute;

pub fn create_deployment(git_url: &str, branch: &str, path: &str, name: &str , uuid: &str) {
    // Get the takoyaki's root directory of the user ($HOME/.takoyaki)
    let takoyaki_root = dirs::home_dir().unwrap().join(".takoyaki");

    // Run the git clone command
    let deployments_dir = takoyaki_root.clone().join("deployments").join(name);
    let build_dir = takoyaki_root.clone().join("build");

    // Create the directory just in case if it is not there 
    create_dir_all(&deployments_dir).unwrap();

    // Execute the git command
    execute(deployments_dir.join(format!("{}.txt" , uuid)), vec!["git" , "clone" , git_url , name] , &build_dir);
    execute(deployments_dir.join(format!("{}.txt" , uuid)), vec!["cargo" , "build" , "--release"] , &build_dir.join(name));
    execute(deployments_dir.join(format!("{}.txt" , uuid)), vec!["mv" , &format!("target/release/{}" , name) , "~/.takoyaki/plugins"] , &build_dir.join(name));
}
