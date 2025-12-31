use clap::{arg, Command};
use tracing::{error, info};

fn cli() -> Command {
    Command::new("tunnel").about("Fast Http Tunnel")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("http").about("Start an http tunnel")
                .arg(arg!(<PORT> "Port to listen on"))
                .arg_required_else_help(true)
        )
}

fn main() {
    tracing_subscriber::fmt::init();

    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("http", sub_matches)) => {
            let port_str = sub_matches.get_one::<String>("PORT").expect("PORT is required");
            let port_n = match port_str.parse::<i32>() {
                Ok(number) => number,
                Err(_e) => {
                    error!("PORT must be a number");
                    return;
                }
            };
            info!("Starting tunnel on port {:?}...", port_n);
        },
        _ => unreachable!()
    };
}
