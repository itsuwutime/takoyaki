use std::{path::PathBuf, io::Result, fs::create_dir_all};

// This is more like kind of a constant paths for the takoyaki plugin builder
pub struct Setup {
    pub deployments_dir: PathBuf,
    pub plugins_dir: PathBuf,
    pub build_dir: PathBuf
}

impl Setup {
    pub fn new() -> Self {
        let takoyaki_root = dirs::home_dir().unwrap().join(".takoyaki");

        Self {
            deployments_dir: takoyaki_root.clone().join("deployments"),
            plugins_dir: takoyaki_root.clone().join("plugins"),
            build_dir: takoyaki_root.clone().join("build")
        }
    }

    pub fn setup(&self) -> Result<()> {
        // Create the deployments dirs
        create_dir_all(&self.deployments_dir)?;

        // Create the plugins directory
        create_dir_all(&self.plugins_dir)?;

        // Create the build directory
        create_dir_all(&self.build_dir)?;

        Ok(())
    }
}
