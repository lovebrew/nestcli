use std::fs::File;
use std::path::{Path, PathBuf};

use anyhow::Result;
use walkdir::WalkDir;
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

use crate::config::bundle::{BundleConfig, CONFIG_NAME};

const BUNDLE_NAME: &str = "bundle.zip";
const IGNORE_NAME: &str = ".bundleignore";

const IGNORE_DATA: &[&str; 7] = &[
    ".git",
    ".gitignore",
    ".gitattributes",
    ".gitmodules",
    ".hg",
    ".svn",
    "bundle.zip",
];

pub struct Bundle {
    cwd: PathBuf,
    config: BundleConfig,
    zip: ZipWriter<File>,
    options: SimpleFileOptions,
    root_includes: Vec<PathBuf>,
}

impl Bundle {
    pub fn new(config: BundleConfig) -> Result<Self> {
        let cwd = std::env::current_dir()?;

        if Path::new(BUNDLE_NAME).exists() {
            std::fs::remove_file(BUNDLE_NAME)?;
        }

        let file = File::create(BUNDLE_NAME)?;
        let zip = ZipWriter::new(file);
        let options = SimpleFileOptions::default();
        let root_includes = Vec::new();

        Ok(Self {
            cwd,
            config,
            zip,
            options,
            root_includes,
        })
    }

    fn collect_root_includes(&mut self) {
        let config = &self.config;

        let mut includes = Vec::new();
        includes.push(self.cwd.join(CONFIG_NAME));

        if Path::new(&self.cwd).join(IGNORE_NAME).exists() {
            includes.push(self.cwd.join(IGNORE_NAME));
        }

        for (_, path) in &config.metadata.icons {
            if Path::new(&self.cwd).join(path).exists() {
                includes.push(self.cwd.join(path));
            }
        }

        includes.sort();
        includes.dedup();
        self.root_includes = includes;
    }

    pub fn add_tree(&mut self) -> Result<()> {
        self.collect_root_includes();

        let cwd = &self.cwd;
        let game_dir = Path::new(&self.config.build.source);

        let walker = WalkDir::new(cwd).into_iter().filter_entry(|entry| {
            if let Some(name) = entry.file_name().to_str() {
                return !IGNORE_DATA.contains(&name);
            }
            true
        });

        for entry in walker.filter_map(|entry| entry.ok()) {
            let zip_path = entry.path().strip_prefix(cwd)?;
            if zip_path.as_os_str().is_empty() {
                continue;
            }

            let is_root_file = self.root_includes.contains(&entry.path().into());
            let file_path = if is_root_file {
                zip_path.to_owned()
            } else {
                game_dir.join(zip_path)
            };

            if entry.path().is_file() {
                let mut file = File::open(entry.path())?;
                self.zip.start_file_from_path(file_path, self.options)?;
                std::io::copy(&mut file, &mut self.zip)?;
            } else {
                self.zip.add_directory_from_path(file_path, self.options)?;
            }
        }
        Ok(())
    }

    pub fn finish(self) -> Result<()> {
        self.zip.finish()?;
        println!("{BUNDLE_NAME} created successfully.");
        Ok(())
    }
}
