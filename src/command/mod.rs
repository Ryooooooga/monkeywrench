pub mod deno;
pub mod node;

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
    Deno(DenoArgs),

    #[command()]
    Node(NodeArgs),
}

// Deno
#[derive(Debug, clap::Args)]
pub struct DenoArgs {
    #[command(subcommand)]
    pub subcommand: DenoSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum DenoSubcommand {
    #[command()]
    Toplevel(DenoTopLevelArgs),

    #[command()]
    Tasks(DenoTasksArgs),
}

#[derive(Debug, clap::Args)]
pub struct DenoTopLevelArgs {
    #[arg(long)]
    pub root: bool,
}

#[derive(Debug, clap::Args)]
pub struct DenoTasksArgs {
    #[arg(long)]
    pub has: Option<String>,
}

// Node.js
#[derive(Debug, clap::Args)]
pub struct NodeArgs {
    #[command(subcommand)]
    pub subcommand: NodeSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum NodeSubcommand {
    #[command()]
    Toplevel(NodeTopLevelArgs),

    #[command()]
    PackageManager,

    #[command()]
    Scripts(NodeScriptsArgs),
}

#[derive(Debug, clap::Args)]
pub struct NodeTopLevelArgs {
    #[arg(long)]
    pub root: bool,
}

#[derive(Debug, clap::Args)]
pub struct NodeScriptsArgs {
    #[arg(long)]
    pub has: Option<String>,
}
