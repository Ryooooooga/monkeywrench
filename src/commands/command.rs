use std::{
    path::{Path, PathBuf},
    process,
};

use super::{CommandAction, CommandArgs};
use crate::path::{
    CARGO_TOML, DENO_JSON, GO_MOD, MAKEFILE, PACKAGE_JSON, PACKAGE_LOCK_JSON, PNPM_LOCK_YAML,
    YARN_LOCK, find_nearest,
};

fn find_build_file(cwd: &Path) -> Option<PathBuf> {
    find_nearest(
        cwd,
        &[PACKAGE_JSON, DENO_JSON, MAKEFILE, CARGO_TOML, GO_MOD],
        crate::path::FindOptions::File,
    )
}

fn get_node_command(cwd: &Path, action: &Option<CommandAction>) -> &'static str {
    let lock_file_path = find_nearest(
        cwd,
        &[PACKAGE_LOCK_JSON, YARN_LOCK, PNPM_LOCK_YAML],
        crate::path::FindOptions::File,
    );

    match lock_file_path {
        Some(path) if path.ends_with(YARN_LOCK) => get_yarn_command(action),
        Some(path) if path.ends_with(PNPM_LOCK_YAML) => get_pnpm_command(action),
        _ => get_npm_command(action),
    }
}

fn get_npm_command(action: &Option<CommandAction>) -> &'static str {
    match action {
        None => "npm",
        Some(CommandAction::Build) => "npm run build",
        Some(CommandAction::Run) => "npm start",
        Some(CommandAction::Test) => "npm test",
        Some(CommandAction::Lint) => "npm run lint",
        Some(CommandAction::Format) => "npm run fmt",
        Some(CommandAction::Generate) => "npm run gen",
    }
}

fn get_yarn_command(action: &Option<CommandAction>) -> &'static str {
    match action {
        None => "yarn",
        Some(CommandAction::Build) => "yarn build",
        Some(CommandAction::Run) => "yarn start",
        Some(CommandAction::Test) => "yarn test",
        Some(CommandAction::Lint) => "yarn lint",
        Some(CommandAction::Format) => "yarn fmt",
        Some(CommandAction::Generate) => "yarn gen",
    }
}

fn get_pnpm_command(action: &Option<CommandAction>) -> &'static str {
    match action {
        None => "pnpm",
        Some(CommandAction::Build) => "pnpm build",
        Some(CommandAction::Run) => "pnpm start",
        Some(CommandAction::Test) => "pnpm test",
        Some(CommandAction::Lint) => "pnpm lint",
        Some(CommandAction::Format) => "pnpm fmt",
        Some(CommandAction::Generate) => "pnpm gen",
    }
}

fn get_deno_command(action: &Option<CommandAction>) -> &'static str {
    match action {
        Some(CommandAction::Build) => "deno task build",
        Some(CommandAction::Run) => "deno task start",
        Some(CommandAction::Test) => "deno test",
        Some(CommandAction::Lint) => "deno lint",
        Some(CommandAction::Format) => "deno fmt",
        Some(CommandAction::Generate) => "deno task gen",
        None => "deno",
    }
}

fn get_make_command(action: &Option<CommandAction>) -> &'static str {
    match action {
        Some(CommandAction::Build) | None => "make",
        Some(CommandAction::Run) => "make run",
        Some(CommandAction::Test) => "make test",
        Some(CommandAction::Lint) => "make lint",
        Some(CommandAction::Format) => "make fmt",
        Some(CommandAction::Generate) => "make gen",
    }
}

fn get_cargo_command(action: &Option<CommandAction>) -> Option<&'static str> {
    match action {
        Some(CommandAction::Build) => Some("cargo build"),
        Some(CommandAction::Run) => Some("cargo run"),
        Some(CommandAction::Test) => Some("cargo test"),
        Some(CommandAction::Lint) => Some("cargo clippy"),
        Some(CommandAction::Format) => Some("cargo fmt"),
        Some(CommandAction::Generate) => None,
        None => Some("cargo"),
    }
}

fn get_go_command(action: &Option<CommandAction>) -> &'static str {
    match action {
        Some(CommandAction::Build) => "go build",
        Some(CommandAction::Run) => "go run .",
        Some(CommandAction::Test) => "go test ./...",
        Some(CommandAction::Lint) => "golangci-lint run",
        Some(CommandAction::Format) => "goimports -w .",
        Some(CommandAction::Generate) => "go gen -v ./...",
        None => "go",
    }
}

pub fn run(args: &CommandArgs) -> anyhow::Result<()> {
    let cwd = std::env::current_dir()?;
    let build_file_path = find_build_file(&cwd);
    let build_file_name = build_file_path
        .as_ref()
        .and_then(|path| path.file_name())
        .and_then(|os_str| os_str.to_str());

    let cmd = match build_file_name {
        Some(PACKAGE_JSON) => Some(get_node_command(&cwd, &args.action)),
        Some(DENO_JSON) => Some(get_deno_command(&args.action)),
        Some(MAKEFILE) => Some(get_make_command(&args.action)),
        Some(CARGO_TOML) => get_cargo_command(&args.action),
        Some(GO_MOD) => Some(get_go_command(&args.action)),
        _ => None,
    };

    match cmd {
        Some(cmd) => println!("{}", cmd),
        None => process::exit(1),
    };

    Ok(())
}
