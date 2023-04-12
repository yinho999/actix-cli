pub mod subcommand;

use clap::Parser;
pub use subcommand::SubCommand;

#[derive(Parser)]
#[clap(version = "1.0")]

pub struct Cli {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}
