use std::collections::HashMap;
use std::path::Path;

use crate::config::bundle::{Build, BundleConfig, CONFIG_NAME, Metadata, PlatformTarget};
use crate::models::bundle::Bundle;
use crate::{confirm, multiselect, txt};

use anyhow::Result;

pub fn generate_bundle_config() -> Result<()> {
    let mut metadata = Metadata {
        name: txt!("Enter game title:", "SuperGame"),
        author: txt!("Enter author name:", "SuperAuthor"),
        description: txt!("Enter game description:", "SuperDescription"),
        version: txt!("Enter game version:", "0.1.0"),
        icons: HashMap::new(),
    };

    let targets = vec!["ctr", "hac", "cafe"];

    let build = Build {
        targets: multiselect!("Select build targets:", targets),
        source: txt!("Enter source directory:", "src"),
        packaged: confirm!(
            "Package the builds?",
            "If yes, the targets will be compiled to binary formats"
        ),
    };

    if build.has_target(PlatformTarget::Ctr) {
        let path = txt!("Nintendo 3DS icon path:", "icon48.png");
        metadata.set_icon(PlatformTarget::Ctr, &path);
    }

    if build.has_target(PlatformTarget::Hac) {
        let path = txt!("Nintendo Switch icon path:", "icon256.jpg");
        metadata.set_icon(PlatformTarget::Hac, &path);
    }

    if build.has_target(PlatformTarget::Cafe) {
        let path = txt!("Nintendo Wiiᵘ icon path:", "icon128.png");
        metadata.set_icon(PlatformTarget::Cafe, &path);
    }

    let bundle_config = BundleConfig { metadata, build };
    let contents = toml::to_string(&bundle_config)?;
    std::fs::write(CONFIG_NAME, contents)?;
    println!("{} created successfully", CONFIG_NAME);

    Ok(())
}

pub fn zip_bundle() -> Result<()> {
    let cwd = std::env::current_dir()?;
    if !Path::new(CONFIG_NAME).exists() {
        anyhow::bail!("Could not find `{CONFIG_NAME}` in `{}`", cwd.display());
    }

    let config_contents = std::fs::read_to_string(CONFIG_NAME)?;
    let config = toml::from_str::<BundleConfig>(&config_contents)?;

    let mut bundle = Bundle::new(config)?;
    bundle.add_tree()?;
    bundle.finish()
}
