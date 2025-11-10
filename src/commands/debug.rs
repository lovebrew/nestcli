use std::{
    io::{Write, stdout},
    net::Ipv4Addr,
};

use anyhow::Result;
use clap::Subcommand;

use crate::config::app::Config;
use crate::models::socket::Socket;
use crate::platforms::addr2line::find_candidate;

#[derive(Subcommand)]
pub enum DebugCmd {
    /// Attach to a remote target
    Attach { address: String },
    /// Debug a local binary using addr2line
    Addr2line {
        filepath: String,
        addresses: Vec<String>,
    },
}

pub fn handle_debug(command: DebugCmd, config: Config) -> Result<()> {
    match command {
        DebugCmd::Attach { address } => {
            let target = match config.get(&address) {
                Some(addr) => *addr,
                None => address.parse::<Ipv4Addr>()?,
            };
            let mut socket = Socket::new((target, config.get_port()))?;
            while let Some(data) = socket.read()? {
                stdout().write_all(data)?;
            }
        }
        DebugCmd::Addr2line {
            filepath,
            addresses,
        } => {
            let filepath = std::path::absolute(filepath)?;
            let candidate = find_candidate(&filepath)?;
            if let Some(candidate) = candidate {
                candidate.command(&filepath, &addresses)?;
            }
        }
    }
    Ok(())
}
