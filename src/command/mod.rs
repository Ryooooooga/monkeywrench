pub mod node;

use clap;

#[derive(Debug, clap::Parser)]
#[command(version, disable_version_flag = true, author, about)]
pub struct Command {
    #[arg(short, long, help = "Print version information", action=clap::ArgAction::Version)]
    pub version: Option<bool>,

    #[command(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    #[command()]
    Node(NodeArgs),
}

#[derive(Debug, clap::Args)]
pub struct NodeArgs {
    #[command(subcommand)]
    pub subcommand: NodeSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum NodeSubcommand {
    #[command()]
    Toplevel,

    #[command()]
    PackageManager,
}
