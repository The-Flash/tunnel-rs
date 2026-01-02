use clap::ArgMatches;

use crate::error::CliError;

#[derive(Debug, Clone, Copy)]
pub struct HttpArgs {
    pub port: u16,
    pub agent_port: u16
}

impl TryFrom<&ArgMatches> for HttpArgs {
    type Error = CliError;
    
    fn try_from(matches: &ArgMatches) -> Result<Self, Self::Error> {
        let port = matches.get_one::<u16>("PORT").ok_or(CliError::RequiredArgument("PORT"))?;
        let agent_port = matches.get_one::<u16>("AGENT_PORT").ok_or(CliError::RequiredArgument("AGENT_PORT"))?;
        if port == agent_port {
            return Err(CliError::ConflictingPorts);
        }
        Ok(Self {
           port: *port,
           agent_port: *agent_port
       }) 
    }
}

impl HttpArgs {
    pub fn bind_addr(&self) -> String {
        format!("{}:{}", "127.0.0.1", self.port)
    }

    pub fn agent_bind_addr(&self) -> String {
        format!("{}:{}", "127.0.0.1", self.agent_port)
    }
}
