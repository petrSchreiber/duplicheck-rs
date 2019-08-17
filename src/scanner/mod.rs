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

pub fn files_to_file_size_tuples(files: Vec<PathBuf>) -> Vec<(PathBuf, u64)> {
    let mut tuples: Vec<(PathBuf, u64)> = vec![];

    for file in &files {
        let file_size = fs::metadata(file);

        if file_size.is_ok() {
            tuples.push((file.to_path_buf(), fs::metadata(file).unwrap().len()));
        }
    }

    tuples
}
