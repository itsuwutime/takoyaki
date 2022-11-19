use crate::Executor;

pub fn create_new_deployment(uuid: &str, git_url: &str, branch: &str , path: &str , name: &str) {
    // Get the root directory for takoyaki
    let root_dirs = dirs::home_dir().unwrap().join(".takoyaki");

    // Get the deployment and the build directory
    let deployment_dir = root_dirs.clone().join("deployments");
    let build_dir = root_dirs.clone().join("build");

    let executor = Executor::new(deployment_dir.join(name).join(format!("{}.txt" , uuid)));

    executor.execute(
        build_dir.clone(),
        vec!["git" , "clone" , git_url , name]
    );

    executor.execute(
        build_dir.join(name),
        vec!["cargo" , "build" , "--release"]
    );
}

