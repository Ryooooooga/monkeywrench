use std::{
    path::{Path, PathBuf},
    process,
};

use super::{CommandAction, CommandArgs};
use crate::path::{
    CARGO_TOML, DENO_JSON, GO_MOD, MAKEFILE, PACKAGE_LOCK_JSON, PNPM_LOCK_YAML, YARN_LOCK,
    find_nearest,
};

#[derive(Debug)]
enum BuildTool {
    Npm,
    Yarn,
    Pnpm,
    Deno,
    Make,
    Cargo,
    Go,
}

fn find_build_file(cwd: &Path) -> Option<PathBuf> {
    find_nearest(
        cwd,
        &[
            PACKAGE_LOCK_JSON,
            YARN_LOCK,
            PNPM_LOCK_YAML,
            DENO_JSON,
            MAKEFILE,
            CARGO_TOML,
            GO_MOD,
        ],
        crate::path::FindOptions::File,
    )
}

pub fn run(args: &CommandArgs) -> anyhow::Result<()> {
    let cwd = std::env::current_dir()?;
    let build_file_path = find_build_file(&cwd);
    let build_file_name = build_file_path
        .as_ref()
        .and_then(|path| path.file_name())
        .and_then(|os_str| os_str.to_str());

    let build_tool = match build_file_name {
        Some(PACKAGE_LOCK_JSON) => BuildTool::Npm,
        Some(YARN_LOCK) => BuildTool::Yarn,
        Some(PNPM_LOCK_YAML) => BuildTool::Pnpm,
        Some(DENO_JSON) => BuildTool::Deno,
        Some(MAKEFILE) => BuildTool::Make,
        Some(CARGO_TOML) => BuildTool::Cargo,
        Some(GO_MOD) => BuildTool::Go,
        _ => process::exit(1),
    };

    let command = match (build_tool, &args.action) {
        (BuildTool::Npm, Some(CommandAction::Build)) => "npm run build",
        (BuildTool::Npm, Some(CommandAction::Run)) => "npm start",
        (BuildTool::Npm, Some(CommandAction::Test)) => "npm test",
        (BuildTool::Npm, Some(CommandAction::Lint)) => "npm run lint",
        (BuildTool::Npm, Some(CommandAction::Format)) => "npm run fmt",
        (BuildTool::Npm, None) => "npm",
        (BuildTool::Yarn, Some(CommandAction::Build)) => "yarn build",
        (BuildTool::Yarn, Some(CommandAction::Run)) => "yarn start",
        (BuildTool::Yarn, Some(CommandAction::Test)) => "yarn test",
        (BuildTool::Yarn, Some(CommandAction::Lint)) => "yarn lint",
        (BuildTool::Yarn, Some(CommandAction::Format)) => "yarn fmt",
        (BuildTool::Yarn, None) => "yarn",
        (BuildTool::Pnpm, Some(CommandAction::Build)) => "pnpm build",
        (BuildTool::Pnpm, Some(CommandAction::Run)) => "pnpm start",
        (BuildTool::Pnpm, Some(CommandAction::Test)) => "pnpm test",
        (BuildTool::Pnpm, Some(CommandAction::Lint)) => "pnpm lint",
        (BuildTool::Pnpm, Some(CommandAction::Format)) => "pnpm fmt",
        (BuildTool::Pnpm, None) => "pnpm",
        (BuildTool::Deno, Some(CommandAction::Build)) => "deno task build",
        (BuildTool::Deno, Some(CommandAction::Run)) => "deno task start",
        (BuildTool::Deno, Some(CommandAction::Test)) => "deno test",
        (BuildTool::Deno, Some(CommandAction::Lint)) => "deno lint",
        (BuildTool::Deno, Some(CommandAction::Format)) => "deno fmt",
        (BuildTool::Deno, None) => "deno",
        (BuildTool::Make, Some(CommandAction::Build) | None) => "make",
        (BuildTool::Make, Some(CommandAction::Run)) => "make run",
        (BuildTool::Make, Some(CommandAction::Test)) => "make test",
        (BuildTool::Make, Some(CommandAction::Lint)) => "make lint",
        (BuildTool::Make, Some(CommandAction::Format)) => "make fmt",
        (BuildTool::Cargo, Some(CommandAction::Build)) => "cargo build",
        (BuildTool::Cargo, Some(CommandAction::Run)) => "cargo run",
        (BuildTool::Cargo, Some(CommandAction::Test)) => "cargo test",
        (BuildTool::Cargo, Some(CommandAction::Lint)) => "cargo clippy",
        (BuildTool::Cargo, Some(CommandAction::Format)) => "cargo fmt",
        (BuildTool::Cargo, None) => "cargo",
        (BuildTool::Go, Some(CommandAction::Build)) => "go build",
        (BuildTool::Go, Some(CommandAction::Run)) => "go run .",
        (BuildTool::Go, Some(CommandAction::Test)) => "go test ./...",
        (BuildTool::Go, Some(CommandAction::Lint)) => "golangci-lint run",
        (BuildTool::Go, Some(CommandAction::Format)) => "goimports -w .",
        (BuildTool::Go, None) => "go",
    };

    println!("{}", command);

    Ok(())
}
