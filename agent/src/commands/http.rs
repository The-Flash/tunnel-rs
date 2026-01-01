use clap::ArgMatches;
use tracing::info;

use crate::{cli::args::HttpArgs, error::CliError};

pub fn run(matches: &ArgMatches) -> Result<(), CliError> {
    let args: HttpArgs = matches.try_into()?;
    info!("Starting tunnel on port {:?}...", args.port);
    info!("Agent is running on port {:?}...", args.agent_port);
    Ok(())
}
