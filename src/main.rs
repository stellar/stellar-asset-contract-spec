use std::{
    error::Error,
    fs::File,
    io::{Cursor, Write},
};

use soroban_sdk::xdr::{Limited, Limits, ReadXdr, WriteXdr};

fn main() -> Result<(), Box<dyn Error>> {
    let spec = soroban_sdk::token::StellarAssetSpec::spec_xdr();
    let mut limited = Limited::new(Cursor::new(spec), Limits::none());
    let entries = soroban_sdk::xdr::ScSpecEntry::read_xdr_iter(&mut limited);

    let mut json_file = File::create("stellar-asset-spec.json")?;
    let mut xdr_file = File::create("stellar-asset-spec.xdr")?;

    for entry in entries {
        let entry = entry?;
        serde_json::to_writer_pretty(&mut json_file, &entry)?;
        xdr_file.write_all(&entry.to_xdr(Limits::none())?)?;
    }

    Ok(())
}
