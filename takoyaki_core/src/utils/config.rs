use crate::Result;

pub struct Config<'a> {
    name: &'a str
}

impl<'a> Config<'a> {
    pub fn new(name: &str) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    #[test] 
    fn get_config() {

    }
}
