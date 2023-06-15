use super::{NodeArgs, NodeScriptsArgs, NodeSubcommand};
use crate::path::find_nearest;
use derive_more::Display;
use serde::Deserialize;
use std::{
    collections::BTreeMap,
    fs::File,
    path::{Path, PathBuf},
};

const PACKAGE_JSON: &str = "package.json";
const PACKAGE_LOCK_JSON: &str = "package-lock.json";
const YARN_LOCK: &str = "yarn.lock";
const PNPM_LOCK_YAML: &str = "pnpm-lock.yaml";

#[derive(Debug, Deserialize)]
struct PackageJson {
    scripts: Option<serde_json::Map<String, serde_json::Value>>,
}

fn find_package_json(cwd: &Path) -> Option<PathBuf> {
    find_nearest(cwd, &[PACKAGE_JSON])
}

fn load_package_json() -> anyhow::Result<Option<PackageJson>> {
    let cwd = std::env::current_dir()?;
    let package_json_path = match find_package_json(&cwd) {
        Some(path) => path,
        None => return Ok(None),
    };

    let file = File::open(package_json_path)?;
    let package_json = serde_json::from_reader(file)?;

    Ok(Some(package_json))
}

fn toplevel() -> anyhow::Result<()> {
    let cwd = std::env::current_dir()?;
    let package_json_path = find_package_json(&cwd);
    let toplevel = match package_json_path {
        Some(path) => path.parent().unwrap().to_path_buf(),
        None => cwd,
    };

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

fn load_scripts() -> anyhow::Result<Option<BTreeMap<String, String>>> {
    let scripts = load_package_json()?.and_then(|p| p.scripts);
    let scripts = match scripts {
        Some(scripts) => scripts,
        None => return Ok(None),
    };

    let valid_scripts = scripts
        .into_iter()
        .filter_map(|(key, value)| Some((key, value.as_str()?.to_string())))
        .collect();

    Ok(Some(valid_scripts))
}

fn scripts(args: &NodeScriptsArgs) -> anyhow::Result<()> {
    let scripts = load_scripts()?;

    if let Some(has) = &args.has {
        let has_script = scripts
            .as_ref()
            .map(|s| s.contains_key(has))
            .unwrap_or(false);

        if !has_script {
            std::process::exit(1);
        }
        return Ok(());
    }

    if let Some(scripts) = &scripts {
        for (key, value) in scripts {
            println!("{key}\t{value}");
        }
    }

    Ok(())
}

pub fn run(args: &NodeArgs) -> anyhow::Result<()> {
    match &args.subcommand {
        NodeSubcommand::Toplevel => toplevel(),
        NodeSubcommand::PackageManager => package_manager(),
        NodeSubcommand::Scripts(args) => scripts(args),
    }
}
