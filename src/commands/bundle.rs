use anyhow::Result;
use clap::Subcommand;

use crate::services::bundle::{generate_bundle_config, zip_bundle};

#[derive(Subcommand, Debug)]
pub enum BundleCmd {
    /// Initialize a Bundle configuration file
    Init,
    /// Create a Bundle from the configuration file
    Create,
}

pub fn handle_bundle(command: BundleCmd) -> Result<()> {
    match command {
        BundleCmd::Init => generate_bundle_config(),
        BundleCmd::Create => zip_bundle(),
    }
}
