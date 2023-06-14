pub mod path;

mod command;

use clap::Parser;
use command::{Command, Subcommand};

fn main() -> anyhow::Result<()> {
    let cmd = Command::parse();
    match &cmd.subcommand {
        Subcommand::Node(args) => command::node::run(args),
    }
}
