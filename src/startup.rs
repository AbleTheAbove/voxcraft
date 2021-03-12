use crate::ROOT_PATH;
use std::path::{Path, PathBuf};
extern crate find_folder;

pub fn addon_folder_exists() -> bool {
    let b: bool = Path::new(ROOT_PATH).is_dir();
    return b;
}

pub fn get_root() -> PathBuf {
    find_folder::Search::ParentsThenKids(3, 3)
        .for_folder(ROOT_PATH)
        .unwrap()
}
