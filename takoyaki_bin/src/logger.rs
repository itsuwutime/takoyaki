use colored::*;

pub struct Logger {
    prefix_character: String
}

impl Logger {
    pub fn new() -> Self {
        Self {
            prefix_character: "==>".to_string()
        }
    }

    fn render(&self , text: ColoredString) {
        println!("{} {}" , self.prefix_character , text)
    }

    pub fn error(&self , msg: &str) {
        self.render(msg.red())
    }

    pub fn success(&self , msg: &str) {
        self.render(msg.green())
    } 

    pub fn warning(&self , msg: &str) {
        self.render(msg.yellow())
    }
}

