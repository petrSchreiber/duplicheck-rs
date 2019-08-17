use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use md5::Digest;

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

pub struct FileSizeInfo {
    pub path: PathBuf,
    pub size: u64,
}

pub fn files_to_file_size_info(files: Vec<PathBuf>) -> Vec<FileSizeInfo> {
    let mut file_size_info: Vec<FileSizeInfo> = vec![];

    for file in &files {
        let file_size = fs::metadata(file);

        if file_size.is_ok() {
            file_size_info.push(FileSizeInfo {
                path: file.to_path_buf(),
                size: fs::metadata(file).unwrap().len(),
            });
        }
    }

    file_size_info
}

pub struct FileMD5Info {
    pub path: PathBuf,
    pub md5: Digest,
}

pub fn file_size_info_to_file_md5_info(file_size_info: &[FileSizeInfo]) -> Vec<FileMD5Info> {
    let mut file_md5_info: Vec<FileMD5Info> = vec![];

    for info in file_size_info {
        let mut file = File::open(&info.path).unwrap();
        let mut contents = vec![];
        file.read_to_end(&mut contents).unwrap();
        let md5 = md5::compute(contents);

        file_md5_info.push(FileMD5Info {
            path: info.path.to_path_buf(),
            md5,
        });
    }

    file_md5_info
}
