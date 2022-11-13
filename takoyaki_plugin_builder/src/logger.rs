use colored::*;

pub struct Logger<'a> {
    prefix_character: &'a str
}

impl<'a> Logger<'a> {
    pub fn new() -> Self {
        Self {
            prefix_character: "==>"
        }
    }

    pub fn success(&self , msg: &str) {
        println!("{} {}" , self.prefix_character.green() , msg)
    }

    pub fn fail(&self , msg: &str) {
        println!("{} {}" , self.prefix_character.red() , msg)
    }
}

