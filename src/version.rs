use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command()]
pub struct Cmd;

impl Cmd {
    pub fn run() {
        println!(
            "stellar-asset-contract-spec {} ({})",
            env!("CARGO_PKG_VERSION"),
            env!("GIT_REVISION"),
        );
    }
}
