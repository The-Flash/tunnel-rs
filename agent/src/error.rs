use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("{0} is required")]
    RequiredArgument(&'static str), 

    #[error("Port conflict detected")]
    ConflictingPorts
}
