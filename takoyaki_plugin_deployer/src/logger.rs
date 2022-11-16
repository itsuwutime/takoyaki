use colored::*;
use chrono::Local;

pub struct Logger {

}

impl Logger {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn render(&self , renderable: ColoredString) {
        let time = Local::now();

        println!("{} {}" , time.format("[%Y-%m-%d][%H:%M:%S]").to_string().blue() , renderable);
    }

    pub fn success(&self , message: &str) {
        self.render(message.green())
    }

    pub fn info(&self , message: &str) {
        self.render(message.cyan())
    }

    pub fn access(&self , message: &str) {
        self.render(message.magenta())
    }

    pub fn fail(&self , message: &str) {
        self.render(message.red())
    }
}
