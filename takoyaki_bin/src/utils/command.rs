use colored::*;

pub struct CommandInfo<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub callback: Box<dyn Fn(Vec<&'a str>)>
}

pub struct Command<'a> {
    commands: Vec<CommandInfo<'a>>
}

impl<'a> Command<'a> {
    pub fn new() -> Self {
        Self {
            commands: vec![]
        }
    }

    pub fn resize_character(&self , original: &'a str , size: usize) -> String {
        //  Get the original length of the string 
        let len = original.len();

        // Get the extra number of characters if needs
        let req = size - len - 3; // 2 is mandatory space on left side
        
        // Get the new string
        return format!("   {}{}" , original , " ".repeat(req));
    }

    pub fn add_command(&mut self , command: CommandInfo<'a>) {
        self.commands.push(command);
    }

    pub fn render(&self) {
        let tab = "    "; // Tab is equal to 4 spaces

        // Print the name and the version
        println!("{} {}" , "takoyaki".green() , option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"));

        // Print the description
        println!("{}" , "Blazingly fast git contribution graph in your terminal");

        // Print the usage
        println!("\n{}" , "USAGE:".yellow());
        println!("{}{}" , tab , "takoyaki [SUBCOMMAND]");

        // Print all the subcommands
        println!("\n{}" , "SUBCOMMANDS:".yellow());

        // Print all the commands
        for subcommand in self.commands.iter() {
            println!("{} {}" , self.resize_character(subcommand.name, 12) , subcommand.description);
        }
    }
}
