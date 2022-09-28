mod send;
pub use send::*;

#[derive(clap::Subcommand)]
pub enum Commands {
    Send(SendCommand),
}

impl Commands {
    pub fn execute(&self) {
        match self {
            Commands::Send(command) => command.execute(),
        }
    }
}

