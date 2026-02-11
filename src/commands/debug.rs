use std::{
    fs::File,
    io::{Write, stdout},
    net::Ipv4Addr,
    path::PathBuf,
};

use anyhow::Result;
use clap::Subcommand;
use indicatif::{ProgressBar, ProgressStyle};

use crate::config::app::Config;
use crate::models::socket::Socket;
use crate::platforms::addr2line::find_candidate;

#[derive(Subcommand)]
pub enum DebugCmd {
    /// Attach to a remote target
    Attach {
        address: String,
        logfile: Option<PathBuf>,
    },
    /// Debug a local binary using addr2line
    Addr2line {
        filepath: String,
        addresses: Vec<String>,
    },
}

fn create_spinner() -> Result<ProgressBar> {
    let progress_bar = ProgressBar::new_spinner();
    let template = ProgressStyle::with_template("Attaching... {spinner}")?
        .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]);
    progress_bar.set_style(template);
    Ok(progress_bar)
}

pub fn handle_debug(command: DebugCmd, config: Config) -> Result<()> {
    match command {
        DebugCmd::Attach { address, logfile } => {
            let target = match config.get(&address) {
                Some(addr) => *addr,
                None => address.parse::<Ipv4Addr>()?,
            };
            let progress = create_spinner()?;
            progress.enable_steady_tick(std::time::Duration::from_millis(120));
            let mut socket = match Socket::new((target, config.get_port())) {
                Ok(socket) => {
                    progress.finish_with_message("Attached.");
                    socket
                }
                Err(e) => {
                    progress.abandon_with_message("Failed to connect.");
                    return Err(e.into());
                }
            };
            let mut file = logfile.map(File::create).transpose()?;
            while let Some(data) = socket.read()? {
                stdout().write_all(data)?;
                if let Some(f) = file.as_mut() {
                    f.write_all(data)?;
                }
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
