// Modules
mod utils;
mod commands;

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
use std::rc::Rc;
use utils::CommandInfo;

#[tokio::main]
async fn main() -> Result<()> {
    let mut command = Box::leak(Box::new(Command::<'static>::new()));

    command.add_commands(vec![
        CommandInfo { 
            name: "init", 
            description: "Initializes a new instance of takoyaki", 
            requires_arg: false,
            callback: Rc::new(|_| { commands::initialize_instance(); })
        },
        // CommandInfo { 
        //     name: "plug", 
        //     description: "Install a new plugin", 
        //     requires_arg: true,
        //     callback: Rc::new(|args: Vec<&str>| {  })
        // },
        // CommandInfo { 
        //     name: "run", 
        //     description: "Execute a specific plugin", 
        //     requires_arg: true,
        //     callback: Rc::new(|args: Vec<&str>| {  })
        // },
        // CommandInfo { 
        //     name: "refresh", 
        //     description: "Refreshes the cache for the plugins", 
        //     requires_arg: true,
        //     callback: Rc::new(|args: Vec<&str>| {  })
        // },
        // CommandInfo { 
        //     name: "unplug", 
        //     description: "Uninstalls a plugin", 
        //     requires_arg: true,
        //     callback: Rc::new(|args: Vec<&str>| {  })
        // },
        // CommandInfo { 
        //     name: "daemon", 
        //     description: "Runs the daemon that updates the cache every hour", 
        //     requires_arg: false,
        //     callback: Rc::new(|args: Vec<&str>| {  })
        // },
        // CommandInfo { 
        //     name: "help", 
        //     description: "Display this help message", 
        //     requires_arg: false,
        //     callback: Rc::new(|args: Vec<&str>| {  })
        // }
    ]);

    command.parse();

    Ok(())
}

