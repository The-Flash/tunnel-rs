use clap::{arg, Command, Arg};

pub fn build() -> Command {
    Command::new("tunnel").about("Fast Http Tunnel")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("http").about("Start an http tunnel")
                .arg(arg!(<PORT> "Port to listen on")
                    .value_parser(clap::value_parser!(u16))
                )
                .arg_required_else_help(true)
                .arg(
                    Arg::new("AGENT_PORT")
                    .help("Port the agent is running on")
                    .short('a')
                    .long("agent-port")
                    .value_parser(clap::value_parser!(u16))
                    .default_value("8000")
                    .default_missing_value("8000")
                )
        )
}

