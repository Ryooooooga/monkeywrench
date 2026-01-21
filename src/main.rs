pub mod path;

mod commands;

use clap::Parser;
use commands::{Command, Subcommand};

fn main() -> anyhow::Result<()> {
    let cmd = Command::parse();
    match &cmd.subcommand {
        Subcommand::Deno(args) => commands::deno::run(args),
        Subcommand::Node(args) => commands::node::run(args),
        Subcommand::Version(args) => commands::version::run(args),
    }
}
