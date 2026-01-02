use clap::ArgMatches;
use tracing::{error, info};
use tokio::net::{TcpListener, TcpStream};

use crate::{cli::args::HttpArgs, error::CliError};

pub async fn run(matches: &ArgMatches) -> Result<(), CliError> {
    let args: HttpArgs = matches.try_into()?;
    info!("Starting tunnel on port {:?}...", args.port);
    info!("Agent is running on port {:?}...", args.agent_port);
    let listener = TcpListener::bind(args.agent_bind_addr()).await?;
    let http_args = args.clone();
    loop {
        match listener.accept().await {
            Ok((mut socket, _addr)) => {
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(&mut socket, http_args).await {
                        error!("failed to handle connection: {}", e);
                    }
                });
            },
            Err(e) => {
                error!("{e}");
                break;
            }
        }
    }
    Ok(())
}

async fn handle_connection(stream: &mut TcpStream, http_args: HttpArgs) -> Result<(), std::io::Error> {
    let mut client_stream = TcpStream::connect(http_args.bind_addr()).await?;
    tokio::io::copy_bidirectional(stream, &mut client_stream).await?;
    Ok(())
}
