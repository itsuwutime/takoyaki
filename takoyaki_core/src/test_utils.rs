#[cfg(test)]
pub mod test_utils {
    use std::path::PathBuf;

    pub fn cache_dir() -> PathBuf {
        dirs::cache_dir().unwrap().join("takoyaki")
    } 
}

