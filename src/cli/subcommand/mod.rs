use clap::Subcommand;
pub mod init;

#[derive(Subcommand)]
pub enum SubCommand {
    #[clap(name = "init")]
    /// Initialize a new project
    Init(init::InitArgs),
}
