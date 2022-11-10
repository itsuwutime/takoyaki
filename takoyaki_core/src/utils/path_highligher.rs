use std::path::PathBuf;

pub fn hint_cache_path(name: &str) -> PathBuf {
    let configured_path = dirs::cache_dir();

    let mut path = match configured_path {
        Some(path) => {
            path
        },
        None => {
            // Hoping that the users have $HOME set
            let mut home_dir = dirs::home_dir().unwrap();

            home_dir.extend([".cache"]);

            home_dir
        }
    };

    path.extend(["takoyaki" , name]);

    // Just to make sure that the directory is present
    std::fs::create_dir_all(&path);

    path
}

