use std::process::Command;

pub fn use_plugin(name: &String) {
    let mut executable = dirs::config_dir().unwrap();

    executable.extend(&["takoyaki" , "plugins" , name.as_ref() , "start"]);

    Command::new("sh")
        .arg("-c")
        .arg(executable)
        .spawn()
        .expect("Error while running the plugin!");
}

