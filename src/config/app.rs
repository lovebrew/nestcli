use std::collections::BTreeMap;
use std::fs;
use std::net::Ipv4Addr;
use std::path::PathBuf;

use anyhow::{Result, bail};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SocketConfig {
    default_port: u16,
}

impl Default for SocketConfig {
    fn default() -> Self {
        Self { default_port: 8000 }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    connections: BTreeMap<String, Ipv4Addr>,
    socket: SocketConfig,
}

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "lovebrew";
const APPLICATION: &str = "nestcli";

const FILE_NAME: &str = "config.toml";

impl Config {
    pub fn path() -> Result<PathBuf> {
        let dirs = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION);
        if let Some(dirs) = dirs {
            return Ok(dirs.config_dir().join(FILE_NAME));
        }
        bail!("Failed to get project directories")
    }

    pub fn reveal(&self) -> Result<()> {
        let path = Self::path()?;
        if let Some(parent) = path.parent() {
            opener::reveal(parent)?;
        }
        Ok(())
    }

    pub fn add(&mut self, name: &str, address: Ipv4Addr) -> Result<()> {
        if name.is_empty() {
            bail!("Name cannot be empty");
        }
        let value = self.connections.insert(name.to_string(), address);
        println!(
            "Connection '{name}' {}",
            value.map_or("added", |_| "updated")
        );
        self.save()
    }

    pub fn get_port(&self) -> u16 {
        self.socket.default_port
    }

    pub fn remove(&mut self, name: &str) -> Result<()> {
        if self.connections.remove(name).is_some() {
            println!("Connection '{name}' removed");
            self.save()?;
        }
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&Ipv4Addr> {
        self.connections.get(name)
    }

    pub fn list(&self) -> Result<()> {
        println!("{:<10} Address", "Name");
        for (name, addr) in &self.connections {
            println!("{name:<10} {addr}");
        }
        Ok(())
    }

    pub fn load() -> Result<Self> {
        let path = Self::path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}
