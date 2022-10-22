mod initer;

use clap::Command;

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
    ;

    let matches = command.get_matches();

    match matches.subcommand() {
        Some(("init" , _)) => {
            initer::initialize_instance().await
        },
        _ => {

        }
    }
}

