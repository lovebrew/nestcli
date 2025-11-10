use anyhow::Result;
use clap::Subcommand;
use std::net::Ipv4Addr;

use crate::config::app::Config;

#[derive(Subcommand, Debug)]
pub enum ConfigCmd {
    /// Add a new connection
    Add { name: String, addr: Ipv4Addr },
    /// Remove a connection
    #[command(visible_alias = "rm", aliases = ["delete", "del"])]
    Remove { name: String },
    /// List all connections
    #[command(visible_alias = "ls", aliases = ["show", "all"])]
    List,
    /// Open the directory to the config file
    Open,
}

pub fn handle_connection(command: ConfigCmd, mut config: Config) -> Result<()> {
    match command {
        ConfigCmd::Add { name, addr } => config.add(&name, addr),
        ConfigCmd::Remove { name } => config.remove(&name),
        ConfigCmd::List => config.list(),
        ConfigCmd::Open => config.reveal(),
    }
}
