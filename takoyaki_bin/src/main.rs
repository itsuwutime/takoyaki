// Modules
mod utils;
mod commands;

// Deps
use crate::utils::Command;
use utils::CommandInfo;

#[tokio::main]
async fn main() {
    let command = Box::leak(Box::new(Command::<'static>::new()));

    command.add_commands(vec![
        CommandInfo { 
            name: "help", 
            description: "Display this help message", 
        },
        CommandInfo { 
            name: "init", 
            description: "Initializes a new instance of takoyaki", 
        },
        CommandInfo { 
            name: "plug", 
            description: "Install a new plugin", 
        },
        CommandInfo { 
            name: "unplug", 
            description: "Uninstalls a plugin", 
        },
        CommandInfo { 
            name: "run", 
            description: "Execute a specific plugin", 
        },
        CommandInfo { 
            name: "refresh", 
            description: "Refreshes the cache for the plugins", 
        },
        CommandInfo { 
            name: "daemon", 
            description: "Runs the daemon that updates the cache every hour", 
        },
    ]);

    let parsed = command.parse();

    match parsed {
        Some(("init" , _)) => {
            commands::initialize_instance();
        },
        Some(("plug" , name)) => {
            commands::plug(name.unwrap()).await;
        },
        Some(("run" , name)) => {
            commands::run(&name.unwrap() , false);
        },
        Some(("refresh" , _)) => {
            commands::refresh();
        },
        Some(("unplug" , name)) => {
            commands::unplug(&name.unwrap());
        },
        Some(("daemon" , _)) => {
            commands::start_daemon();
        },
        Some(("help" , _)) => {
            command.render();
        },
        _ => {

        }
    }
}

