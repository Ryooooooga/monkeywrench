use std::path::{Path, PathBuf};

pub const PACKAGE_JSON: &str = "package.json";
pub const PACKAGE_LOCK_JSON: &str = "package-lock.json";
pub const YARN_LOCK: &str = "yarn.lock";
pub const PNPM_LOCK_YAML: &str = "pnpm-lock.yaml";
pub const DENO_JSON: &str = "deno.json";

#[derive(Debug)]
pub enum FindOptions {
    Anything,
    File,
    Directory,
}

pub fn find_nearest(dir: &Path, files_to_find: &[&str], opt: FindOptions) -> Option<PathBuf> {
    let mut dir = dir;

    loop {
        for file in files_to_find.iter() {
            let path = dir.join(file);
            match opt {
                FindOptions::Anything if path.exists() => return Some(path),
                FindOptions::File if path.is_file() => return Some(path),
                FindOptions::Directory if path.is_dir() => return Some(path),
                _ => {}
            }
        }

        match dir.parent() {
            Some(parent) => dir = parent,
            None => break,
        };
    }
    None
}
