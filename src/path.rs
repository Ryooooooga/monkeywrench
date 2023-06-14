use std::path::{Path, PathBuf};

pub fn find_nearest(dir: &Path, files_to_find: &[&str]) -> Option<PathBuf> {
    let mut dir = dir;

    loop {
        for file in files_to_find.iter() {
            let path = dir.join(file);
            if path.exists() {
                return Some(path);
            }
        }

        match dir.parent() {
            Some(parent) => dir = parent,
            None => break,
        };
    }
    None
}
