use colored::*;

#[derive(Clone)]
pub struct CommandInfo<'a> {
    pub name: &'a str,
    pub description: &'a str,
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
        let req = size - len - 4; // 2 is mandatory space on left side

        // Get the new string
        return format!("    {}{}" , original , " ".repeat(req));
    }

    pub fn add_commands(&mut self , commands: Vec<CommandInfo<'a>>) {
        self.commands.extend(commands);
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
            println!("{} {}" , self.resize_character(subcommand.name, 14).green() , subcommand.description);
        }
    }

    fn get_command_with_name(&'a self , subcommand: &String) -> Option<&CommandInfo> {
        let matches: Vec<&CommandInfo<'a>> = self.commands.iter().filter(|x| { 
            x.name == subcommand
        }).collect();

        matches.into_iter().nth(0)
    }

    fn exists(&'a self , subcommand: &String) -> bool {
        let command = self.get_command_with_name(subcommand);

        command.is_some()
    }

    pub fn send_calls(&'a self , args: Vec<String>) -> (&str , Option<String>) {
        let subcommand = args.iter().nth(1).unwrap(); // It is sure that it is gonna contain something at 1st position

        if !self.exists(subcommand) {
            println!("{}: Found argument '{}' which wasn't expected, or is not valid in this context" , "error".red().bold() , subcommand.yellow());
            println!("\nFor more information try {}" , "--help".green());

            std::process::exit(1)
        }

        // Get the command 
        let command = self.get_command_with_name(subcommand).unwrap();

        return (command.name , args.clone().into_iter().nth(2))
    }

    pub fn parse(&'a self) -> Option<(&str , Option<String>)> {
        let args: Vec<String> = std::env::args().collect();

        match args.len() {
            1 => { // 0 is just impossible
                self.render();

                return None
            },
            _ => {
                Some(self.send_calls(args))
            }
        }
    }
}

