use std::path::{Path, PathBuf};

pub fn find_nearest(dir: &Path, files_to_find: &[&str]) -> Option<PathBuf> {
    let mut d = dir.clone();

    loop {
        for file in files_to_find.iter() {
            let path = d.join(file);
            if path.exists() {
                return Some(path);
            }
        }

        match d.parent() {
            Some(parent) => d = parent,
            None => break,
        };
    }
    None
}
