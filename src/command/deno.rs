use super::{DenoArgs, DenoSubcommand, DenoTasksArgs, DenoTopLevelArgs};
use crate::path::{FindOptions, find_nearest};
use serde::Deserialize;
use std::{
    collections::BTreeMap,
    fs::File,
    path::{Path, PathBuf},
};

const DENO_JSON: &str = "deno.json";

#[derive(Debug, Deserialize)]
struct DenoJson {
    tasks: Option<serde_json::Map<String, serde_json::Value>>,
}

fn find_deno_json(cwd: &Path) -> Option<PathBuf> {
    find_nearest(cwd, &[DENO_JSON], FindOptions::File)
}

fn find_top_level(cwd: &Path) -> Option<PathBuf> {
    Some(find_deno_json(cwd)?.parent().unwrap().to_path_buf())
}

fn load_deno_json() -> anyhow::Result<Option<DenoJson>> {
    let cwd = std::env::current_dir()?;
    let deno_json_path = match find_deno_json(&cwd) {
        Some(path) => path,
        None => return Ok(None),
    };

    let file = File::open(deno_json_path)?;
    let deno_json = serde_json::from_reader(file)?;

    Ok(Some(deno_json))
}

fn toplevel(_args: &DenoTopLevelArgs) -> anyhow::Result<()> {
    let cwd = std::env::current_dir()?;
    let toplevel = find_top_level(&cwd).unwrap_or(cwd);

    println!("{}", toplevel.display());

    Ok(())
}

fn load_tasks() -> anyhow::Result<Option<BTreeMap<String, String>>> {
    let tasks = load_deno_json()?.and_then(|p| p.tasks);
    let tasks = match tasks {
        Some(tasks) => tasks,
        None => return Ok(None),
    };

    let valid_tasks = tasks
        .into_iter()
        .filter_map(|(key, value)| Some((key, value.as_str()?.to_string())))
        .collect();

    Ok(Some(valid_tasks))
}

fn tasks(args: &DenoTasksArgs) -> anyhow::Result<()> {
    let scripts = load_tasks()?;

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

pub fn run(args: &DenoArgs) -> anyhow::Result<()> {
    match &args.subcommand {
        DenoSubcommand::Toplevel(args) => toplevel(args),
        DenoSubcommand::Tasks(args) => tasks(args),
    }
}
