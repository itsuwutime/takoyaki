mod initer;
mod unplug;
mod use_plugin;
mod plug;

use clap::{Command, Arg};

#[tokio::main]
async fn main() {
    let command = Command::new("takoyaki")
        .arg_required_else_help(true)
        .about("takoyaki - Get your git contribution gaph in your terminal")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"))
        .subcommand(
            Command::new("init")
                .about("Initializes a new instance of takoyaki to get started")
        )
        .subcommand(
            Command::new("plug")
                .about("Installs a new plugin")
                .arg(
                    Arg::new("branch")
                        .default_value("main")
                        .short('b')
                        .long("branch")
                        .help("Use a custom branch")
                )
                .arg(
                    Arg::new("path")
                        .default_value("/")
                        .short('p')
                        .long("path")
                        .help("Use a specific directory")
                )
                .arg(
                    Arg::new("repo")
                        .required(true)
                        .short('r')
                        .long("repo")
                        .help("Repository name [Only add the username and the repo name, example: kyeboard/takoyaki] ")
                )
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("use")
                .about("Uses a plugin")
                .arg(
                    Arg::new("plugin")
                        .required(true)
                        .short('p')
                        .long("plugin")
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
                        .short('p')
                        .long("plugin")
                        .help("The name of the plugin to uninstall")
                )
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("clean")
                .about("Removes all the cache so that on the next run plugins can synx with the new data")
        )
    ;

    let matches = command.get_matches();

    match matches.subcommand() {
        Some(("init" , _)) => {
            initer::initialize_instance().await
        },
        Some(("plug" , sub_matches)) => {
            plug::plug(
                sub_matches.get_one::<String>("repo").unwrap(),
                sub_matches.get_one::<String>("branch").unwrap(),
                sub_matches.get_one::<String>("path").unwrap()
            ).await
        },
        Some(("use" , sub_matches)) => {
            use_plugin::use_plugin(sub_matches.get_one::<String>("plugin").unwrap());
        },
        Some(("unplug" , sub_matches)) => {
            unplug::unplug(sub_matches.get_one::<String>("plugin").unwrap());
        },
        Some(("" , sub_matches)) => {
            unplug::unplug(sub_matches.get_one::<String>("plugin").unwrap());
        },
        _ => {

        }
    }
}

