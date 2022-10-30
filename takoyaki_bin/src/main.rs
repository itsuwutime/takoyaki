mod initer;
mod unplug;
mod logger;
mod helpers;
mod metadata;
mod use_plugin;
mod plug;

use clap::{Command, Arg, ArgAction};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let command = Command::new("takoyaki")
        .arg_required_else_help(true)
        .about("takoyaki - Get your git contribution graph in your terminal")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"))
        .subcommand(
            Command::new("init")
                .about("Initialize an instance of takoyaki")
        )
        .subcommand(
            Command::new("plug")
                .about("Installs a new plugin")
                .arg(
                    Arg::new("name")
                        .required(true)
                        .action(ArgAction::Set)
                        .help("Name of the plugin to install")
                )
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("use")
                .about("Uses a plugin")
                .arg(
                    Arg::new("plugin")
                        .required(true)
                        .action(ArgAction::Set)
                        .help("The name of the plugin to use")
                )
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("unplug")
                .about("Uninstalls a plugin")
                .arg(
                    Arg::new("plugin")
                        .required(true)
                        .action(ArgAction::Set)
                        .help("The name of the plugin to uninstall")
                )
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("clean")
                .about("Cleans up the cache (warning: It might take a bit for the plugins to run for the first time after clean)")
        )
    ;

    let matches = command.get_matches();

    match matches.subcommand() {
        Some(("init" , _)) => {
            initer::initialize_instance().await?
        },
        Some(("plug" , sub_matches)) => {
            plug::plug(
                sub_matches.get_one::<String>("name").unwrap().to_owned() // It is required so it should contain something
            ).await?
        },
        Some(("use" , sub_matches)) => {
            use_plugin::use_plugin(sub_matches.get_one::<String>("plugin").unwrap())?;
        },
        Some(("unplug" , sub_matches)) => {
            unplug::unplug(sub_matches.get_one::<String>("plugin").unwrap())?;
        },
        _ => {

        }
    }

    Ok(())
}

