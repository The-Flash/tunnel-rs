mod config;
mod cli;
mod error;
mod commands;

use tracing::error;

fn main() {
    config::tracing::init();

    let matches = cli::build().get_matches();

    if let Err(e) = commands::dispatch(matches) {
        error!("{e}");
        std::process::exit(1);
    }
}
