use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize, Serialize};

pub const CONFIG_NAME: &str = "lovebrew.toml";

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum PlatformTarget {
    Ctr,
    Hac,
    Cafe,
}

impl FromStr for PlatformTarget {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ctr" => Ok(Self::Ctr),
            "hac" => Ok(Self::Hac),
            "cafe" => Ok(Self::Cafe),
            _ => Err("No such match".into()),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Metadata {
    pub name: String,
    pub author: String,
    pub description: String,
    pub version: String,
    pub icons: HashMap<PlatformTarget, String>,
}

impl Metadata {
    pub fn set_icon(&mut self, platform: PlatformTarget, path: &str) {
        self.icons.insert(platform, String::from(path));
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Build {
    pub targets: Vec<PlatformTarget>,
    pub source: String,
    pub packaged: bool,
}

impl Build {
    pub fn has_target(&self, target: PlatformTarget) -> bool {
        self.targets.contains(&target)
    }
}

#[derive(Serialize, Deserialize)]
pub struct BundleConfig {
    pub metadata: Metadata,
    pub build: Build,
}
