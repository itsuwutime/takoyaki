// Specifies all the paths that are being used by takoyaki

use std::path::PathBuf;


// Main struct
#[derive(Clone)]
pub struct Path {
    pub config_dir: PathBuf,
    pub cache_dir: PathBuf
}

impl Path {

}

