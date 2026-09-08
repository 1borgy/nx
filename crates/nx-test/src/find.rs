use std::{
    env,
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

fn assets_subdir(subdir: impl AsRef<Path>) -> PathBuf {
    env::current_dir()
        .expect("could not get cwd")
        .join("..")
        .join("..")
        .join("assets")
        .join(subdir)
}

fn walk_assets(dir: impl AsRef<Path>) -> Vec<PathBuf> {
    let dir = dir.as_ref();
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .map(|entry| PathBuf::from(entry.path()))
        .filter(|path| path.is_file())
        .collect()
}

pub fn find_assets(subdir: impl AsRef<Path>) -> Vec<PathBuf> {
    walk_assets(assets_subdir(subdir))
}
