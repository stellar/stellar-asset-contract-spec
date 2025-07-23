use soroban_sdk::xdr::{self, Limited, Limits, ReadXdr, WriteXdr};
use std::io::{self, stdout};
use std::io::{Cursor, Write};

use clap::{Args, ValueEnum};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Xdr(#[from] xdr::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    Io(#[from] io::Error),
}

#[derive(Args, Debug, Clone)]
#[command()]
pub struct Cmd {
    #[arg(long)]
    format: Format,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, ValueEnum)]
pub enum Format {
    Json,
    Xdr,
}

impl Cmd {
    pub fn run(&self) -> Result<(), Error> {
        let spec = soroban_sdk::token::StellarAssetSpec::spec_xdr();
        let mut limited = Limited::new(Cursor::new(spec), Limits::none());
        let entries = soroban_sdk::xdr::ScSpecEntry::read_xdr_iter(&mut limited);

        match self.format {
            Format::Json => {
                let mut collected_entries = Vec::new();
                for entry in entries {
                    let entry = entry?;
                    collected_entries.push(entry);
                }
                serde_json::to_writer_pretty(stdout(), &collected_entries)?;
            }
            Format::Xdr => {
                for entry in entries {
                    let entry = entry?;
                    stdout().write_all(&entry.to_xdr(Limits::none())?)?;
                }
            }
        }

        Ok(())
    }
}
