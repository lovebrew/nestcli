use std::fs::File;
use std::io::{BufReader, Write, stdout};
use std::path::{Path, PathBuf};
use std::process::Command;

use aho_corasick::AhoCorasick;
use anyhow::Result;

pub struct Candidate {
    binary: &'static str,
    pub magic: &'static [u8],
    args: &'static [&'static str],
    search_path: &'static str,
    device: &'static str,
}

const CANDIDATES: [Candidate; 3] = [
    Candidate {
        binary: "arm-none-eabi-addr2line",
        magic: b"3dsx_crt0.o",
        args: &["arm"],
        search_path: "devkitARM/bin",
        device: "Nintendo 3DS",
    },
    Candidate {
        binary: "aarch64-none-elf-addr2line",
        magic: b"switch_crt0.o",
        args: &[""],
        search_path: "devkitA64/bin",
        device: "Nintendo Switch",
    },
    Candidate {
        binary: "powerpc-eabi-addr2line",
        magic: b"crt0_rpx.o",
        args: &[""],
        search_path: "devkitPPC/bin",
        device: "Nintendo Wiiᵘ",
    },
];

pub fn find_candidate(filepath: &Path) -> Result<Option<&Candidate>> {
    let reader = match File::open(filepath) {
        Ok(file) => BufReader::new(file),
        Err(_) => anyhow::bail!("Failed to open file: {filepath:#?}"),
    };

    let patterns = CANDIDATES.iter().map(|c| c.magic).collect::<Vec<&[u8]>>();
    let ac = AhoCorasick::new(&patterns)?;

    if let Some(matched) = ac.stream_find_iter(reader).flatten().next() {
        let index = matched.pattern().as_usize();
        return Ok(Some(&CANDIDATES[index]));
    }
    Ok(None)
}

impl Candidate {
    pub fn command(&self, filepath: &Path, addresses: &[String]) -> Result<()> {
        if !filepath.exists() || !filepath.is_file() {
            anyhow::bail!("File not found: {filepath:#?}");
        }

        if addresses.is_empty() {
            anyhow::bail!("No addresses provided");
        }

        let binary_name = match std::env::var("DEVKITPRO") {
            Ok(val) => PathBuf::from(val).join(self.search_path).join(self.binary),
            Err(_) => PathBuf::from(self.binary),
        };

        if which::which(&binary_name).is_err() {
            anyhow::bail!("{} not found", self.binary);
        }

        println!("[{}: {}]", self.device, self.binary);

        let mut command = Command::new(binary_name);
        command
            .arg("-aipfCe")
            .args(self.args)
            .arg("-e")
            .arg(filepath);

        for address in addresses {
            command.arg(address);
        }

        let output = match command.output() {
            Ok(output) => output.stdout,
            Err(e) => anyhow::bail!("Failed to execute addr2line: {e}"),
        };

        stdout().write_all(&output)?;
        Ok(())
    }
}
