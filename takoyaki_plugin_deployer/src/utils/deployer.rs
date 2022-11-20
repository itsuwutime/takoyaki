use std::fs::create_dir_all;

use crate::Executor;

pub fn create_new_deployment(uuid: &str, git_url: &str, branch: &str , path: &str , name: &str) {
    // Get the root directory for takoyaki
    let root_dirs = dirs::home_dir().unwrap().join(".takoyaki");

    // Get the deployment and the build directory
    let deployment_dir = root_dirs.clone().join("deployments").join(name);
    let build_dir = root_dirs.clone().join("build");
    let plugin_dir = root_dirs.clone().join("plugins");

    // Create the deployments directory
    create_dir_all(&deployment_dir).unwrap();

    let executor = Executor::new(deployment_dir.join(format!("{}.txt" , uuid)));

    executor.execute(
        build_dir.clone(),
        vec!["git" , "clone" , "-b" , branch , git_url , name]
    );

    executor.execute(
        build_dir.join(name).join(path),
        vec!["cargo" , "build" , "--release"]
    );

    executor.execute(
        build_dir.join(name).join(path).join("target").join("release"),
        vec!["mv" , name , plugin_dir.to_str().unwrap()]
    );
}

