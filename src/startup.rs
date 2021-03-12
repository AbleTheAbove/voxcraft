use crate::ROOT_PATH;
use home;
use std::path::{Path, PathBuf};

pub fn addon_folder_exists() -> bool {
    let b: bool = Path::new(ROOT_PATH).is_dir();
    home_dir();
    return b;
}

fn home_dir() -> Option<PathBuf> {
    let home_dir = home::home_dir();
    home_dir
}
