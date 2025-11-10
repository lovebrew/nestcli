mod commands;
mod config;
mod models;
mod platforms;
mod prompts;
mod services;

use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::{bundle::BundleCmd, conn::ConfigCmd, debug::DebugCmd};
use config::app::Config;

use commands::{bundle::handle_bundle, conn::handle_connection, debug::handle_debug};

#[derive(Parser)]
#[command(author="support@lovebrew.org", version, about, long_about = None)]
#[command(name = "nestcli")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add, remove, or list configured target devices
    #[command(alias = "c")]
    Config {
        #[command(subcommand)]
        command: ConfigCmd,
    },
    /// Tools for debugging builds and resolving symbols
    #[command(alias = "dbg")]
    Debug {
        #[command(subcommand)]
        command: DebugCmd,
    },
    /// Bundle utilization commands
    #[command(alias = "b")]
    Bundle {
        #[command(subcommand)]
        command: BundleCmd,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::load()?;

    match cli.command {
        Commands::Config { command } => handle_connection(command, config),
        Commands::Debug { command } => handle_debug(command, config),
        Commands::Bundle { command } => handle_bundle(command),
    }
}
