use clap::{arg, Arg, Command};
use tracing::info;

fn cli() -> Command {
    Command::new("tunnel").about("Fast Http Tunnel")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("http").about("Start an http tunnel")
                .arg(arg!(<PORT> "Port to listen on")
                    .value_parser(clap::value_parser!(i32))
                )
                .arg_required_else_help(true)
                .arg(
                    Arg::new("AGENT_PORT")
                    .help("Port the agent is running on")
                    .short('a')
                    .long("agent-port")
                    .value_parser(clap::value_parser!(i32))
                    .default_value("8000")
                    .default_missing_value("8000")
                )
        )
}

fn main() {
    tracing_subscriber::fmt::init();

    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("http", sub_matches)) => {
            let port = sub_matches.get_one::<i32>("PORT").expect("PORT is required");
            info!("Starting tunnel on port {:?}...", port);
            let agent_port = sub_matches.get_one::<i32>("AGENT_PORT").expect("AGENT_PORT has a default value");
           info!("Agent is running on port {:?}...", agent_port);
        },
        _ => unreachable!()
    };
}
