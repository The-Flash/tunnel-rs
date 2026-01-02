use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("{0} is required")]
    RequiredArgument(&'static str), 

    #[error("Port conflict detected")]
    ConflictingPorts,

    #[error("IO error: {0}")]
    Io(std::io::Error)
}

impl From<std::io::Error> for CliError {
    fn from(value: std::io::Error) -> Self {
       Self::Io(value) 
    }
}
