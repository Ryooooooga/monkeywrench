use super::{NodeArgs, NodeSubcommand};
use crate::path::find_nearest;
use derive_more::Display;
use std::path::PathBuf;

const PACKAGE_JSON: &str = "package.json";
const PACKAGE_LOCK_JSON: &str = "package-lock.json";
const YARN_LOCK: &str = "yarn.lock";
const PNPM_LOCK_YAML: &str = "pnpm-lock.yaml";

fn find_toplevel() -> anyhow::Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    let package_json = find_nearest(&cwd, &[PACKAGE_JSON]);

    let toplevel = match package_json {
        Some(package_json) => package_json.parent().unwrap().to_path_buf(),
        None => cwd,
    };
    Ok(toplevel)
}

fn toplevel() -> anyhow::Result<()> {
    let toplevel = find_toplevel()?;
    println!("{}", toplevel.display());

    Ok(())
}

#[derive(Debug, Display)]
enum PackageManager {
    #[display(fmt = "npm")]
    Npm,
    #[display(fmt = "yarn")]
    Yarn,
    #[display(fmt = "pnpm")]
    Pnpm,
}

fn detect_package_manager() -> anyhow::Result<PackageManager> {
    let cwd = std::env::current_dir()?;
    let lock_path = find_nearest(&cwd, &[PACKAGE_LOCK_JSON, YARN_LOCK, PNPM_LOCK_YAML]);

    let package_manager = match lock_path {
        Some(p) if p.ends_with(YARN_LOCK) => PackageManager::Yarn,
        Some(p) if p.ends_with(PNPM_LOCK_YAML) => PackageManager::Pnpm,
        _ => PackageManager::Npm,
    };

    Ok(package_manager)
}

fn package_manager() -> anyhow::Result<()> {
    let package_manager = detect_package_manager()?;
    println!("{}", package_manager);

    Ok(())
}

pub fn run(args: &NodeArgs) -> anyhow::Result<()> {
    match &args.subcommand {
        NodeSubcommand::Toplevel => toplevel(),
        NodeSubcommand::PackageManager => package_manager(),
    }
}
