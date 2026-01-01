use clap::{arg, Arg, Command};
use tracing::{error, info};

fn cli() -> Command {
    Command::new("tunnel").about("Fast Http Tunnel")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("http").about("Start an http tunnel")
                .arg(arg!(<PORT> "Port to listen on"))
                .arg_required_else_help(true)
                .arg(
                    Arg::new("AGENT_PORT")
                    .help("Port the agent is running on")
                    .short('a')
                    .long("agent-port")
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
            let port_str = sub_matches.get_one::<String>("PORT").expect("PORT is required");
            let port_n = match port_str.parse::<i32>() {
                Ok(number) => number,
                Err(_e) => {
                    error!("PORT must be a number");
                    return;
                }
            };
            info!("Starting tunnel on port {:?}...", port_n);
            let mut agent_port = 8000;
            if let Some(agent_port_str) = sub_matches.get_one::<String>("AGENT_PORT") {
                agent_port = match agent_port_str.parse::<i32>() {
                    Ok(number) => number,
                    Err(_e) => {
                        error!("AGENT_PORT must be a number");
                        return;
                    }
                }
            }
           info!("Agent is running on port {:?}...", agent_port);
        },
        _ => unreachable!()
    };
}
