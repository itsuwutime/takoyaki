mod initer;
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
        _ => {

        }
    }
}

