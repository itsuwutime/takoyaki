use std::{fs::File, io};

use colored::*;

pub async fn initialize_instance() {
    let config_dir = dirs::config_dir().unwrap().join("takoyaki");
    let config_path = config_dir.join("config.toml");

    std::fs::create_dir_all(&config_dir).expect("Error while creating a new directory");

    println!("{} {}" , "==>".green() , "Initializing a new instace of takoyaki...".white());
    println!("{} {} {}..." , "==>".green() , "Populating default config for takoyaki at".white() , config_path.display().to_string().bold());

    // Download file
    let req = reqwest::get("https://raw.githubusercontent.com/kyeboard/takoyaki/main/default.toml").await.expect("Error while making a request").bytes().await.expect("Error while parsing the response!");

    let mut out = File::create(config_path).expect("Error file creating the config file!");

    io::copy(&mut req.as_ref(), &mut out).expect("Error while writing to file!");

    println!("{} {}" , "==>".green() , "Successfully populated default config!".white());
    println!("{} {}" , "==>".green() , "Adding touches...".white());

    // Create all the directory 
    std::fs::create_dir_all(&config_dir.join("plugins")).expect("Error while creating a directory");

    println!("{} {}" , "==>".green() , "Done!".white());
}
