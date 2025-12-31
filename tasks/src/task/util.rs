use std::{
    env,
    path::{Path, PathBuf},
};

pub fn get_project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .expect("Failed to get the project root!")
        .to_path_buf()
}
