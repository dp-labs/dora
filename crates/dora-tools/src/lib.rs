use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

pub fn find_all_json_tests(path: &Path) -> Vec<PathBuf> {
    let mut paths = if path.is_file() {
        vec![path.to_path_buf()]
    } else {
        WalkDir::new(path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.path().extension() == Some("json".as_ref()))
            .map(DirEntry::into_path)
            .collect()
    };
    paths.sort();
    paths
}
