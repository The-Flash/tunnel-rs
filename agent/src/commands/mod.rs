use clap::ArgMatches;

use crate::error::CliError;

mod http;

pub fn dispatch(matches: ArgMatches) -> Result<(), CliError> {
    match matches.subcommand() {
        Some(("http", sub)) => http::run(sub),
        _ => unreachable!()
    }
}
