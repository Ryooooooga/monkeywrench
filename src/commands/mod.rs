pub mod deno;
pub mod node;
pub mod version;

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

    #[command()]
    Version(VersionArgs),
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
pub struct DenoTopLevelArgs {}

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

// Version
#[derive(Debug, clap::Args)]
pub struct VersionArgs {
    #[command(subcommand)]
    pub subcommand: VersionSubcommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum VersionSubcommand {
    #[command()]
    Inc(VersionIncArgs),
}

#[derive(Debug, clap::Args)]
pub struct VersionIncArgs {
    #[arg(long)]
    pub level: u32,

    #[arg()]
    pub version: String,
}
