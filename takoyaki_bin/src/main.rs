// Modules
mod utils;
mod initer;
mod refresh;
mod unplug;
mod logger;
mod daemon;
mod helpers;
mod metadata;
mod use_plugin;
mod plug;

// Deps
use crate::utils::Command;
use anyhow::Result;
use utils::CommandInfo;

#[tokio::main]
async fn main() -> Result<()> {
    let mut command = Command::new();

    command.add_commands(vec![
        CommandInfo { 
            name: "init", 
            description: "Initializes a new instance of takoyaki", 
            callback: Box::new(|args: Vec<&str>| {  })
        },
        CommandInfo { 
            name: "plug", 
            description: "Install a new plugin", 
            callback: Box::new(|args: Vec<&str>| {  })
        },
        CommandInfo { 
            name: "run", 
            description: "Execute a specific plugin", 
            callback: Box::new(|args: Vec<&str>| {  })
        },
        CommandInfo { 
            name: "unplug", 
            description: "Uninstalls a plugin", 
            callback: Box::new(|args: Vec<&str>| {  })
        },
        CommandInfo { 
            name: "daemon", 
            description: "Runs the daemon that updates the cache every hour", 
            callback: Box::new(|args: Vec<&str>| {  })
        },
        CommandInfo { 
            name: "help", 
            description: "Display this help message", 
            callback: Box::new(|args: Vec<&str>| {  })
        }
    ]);

    command.render();

    Ok(())
}

