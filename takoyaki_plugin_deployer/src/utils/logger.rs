use chrono::Local;
use colored::*;

pub struct Logger {

}

impl Logger {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn render(&self , renderable: ColoredString) {
        let time = Local::now().format("[%Y-%m-%d][%H:%M:%S]").to_string();

        println!("{} {}" , time.blue() , renderable);
    }

    pub fn success(&self , msg: &str) {
        self.render(msg.green())
    }

    pub fn warning(&self , msg: &str) {
        self.render(msg.yellow())
    }

    pub fn error(&self , msg: &str) {
        self.render(msg.red())
    }
}

