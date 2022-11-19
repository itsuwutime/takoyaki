use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Command {
    fn new() -> Self where Self: Sized;
    fn prefix(&self) -> &str;
    async fn respond(&self , args: Vec<&str>) -> Result<String>;
}

pub struct CommandManager<'a> {
    commands: Vec<Box<dyn Command + Send + Sync + 'a>>
}

impl<'a> CommandManager<'a> {
    pub fn new() -> Self {
        Self {
            commands: vec![]
        }
    }

    pub fn register_command(&mut self , command: Box<dyn Command + Send + Sync + 'a>) {
        self.commands.push(command)
    }

    pub async fn parse_from_raw(&self , raw: &str) -> String {
        // Get the args and the command that the user wants to run
        let mut args: Vec<&str> = raw.trim().split_whitespace().collect();
        let raw_command = args.remove(0);

        // Get the command handler
        let handler = self.commands.iter().find(|i| {
            i.prefix() == raw_command
        });

        // Execute the handler if it is present
        if handler.is_some() {
            handler.unwrap().respond(args).await.unwrap()
        } else {
            "Command not found!".to_string()
        }
    }
}
