use std::path::PathBuf;

use std::fs;

pub fn enumerate_files_in_dir(directory: PathBuf) -> Vec<PathBuf> {
    let mut result: Vec<PathBuf> = vec![];

    if directory.is_dir() {
        let directory_read_result = fs::read_dir(directory);
        if directory_read_result.is_ok() {
            for entry in directory_read_result.unwrap() {
                if entry.is_ok() {
                    let path = entry.unwrap().path();
                    if path.is_dir() {
                        result.append(&mut enumerate_files_in_dir(path));
                    } else {
                        result.push(path);
                    }
                }
            }
        }
    }

    result
}
