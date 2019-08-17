use crate::scanner::FileMD5Info;
use std::path::PathBuf;

use structopt::StructOpt;

mod scanner;

#[derive(Debug, StructOpt)]
#[structopt(name = "duplicheck-rs", about = "Tool for finding duplicate files.")]
struct CommandLineParams {
    /// Directory to be scanned
    directory: String,
}

fn main() {
    let command_line_params = CommandLineParams::from_args();

    println!("{}", "-".repeat(80));
    println!("{}", command_line_params.directory);
    println!("{}", "-".repeat(80));

    let mut directory_path = PathBuf::new();
    directory_path.push(command_line_params.directory);

    let all_files = scanner::enumerate_files_in_dir(directory_path);
    let file_size_pairs = scanner::files_to_file_size_info(all_files);
    let mut file_md5_pairs = scanner::file_size_info_to_file_md5_info(&file_size_pairs);

    list_duplicate_files(&mut file_md5_pairs);
}

fn list_duplicate_files(file_md5_info: &mut [scanner::FileMD5Info]) {
    // Sorting by MD5 first
    file_md5_info.sort_by(|a, b| a.md5.cmp(&b.md5));

    // Creating peekable iterator
    let mut file_iter = file_md5_info.iter().peekable();

    while let Some(&item) = file_iter.peek() {
        file_iter.next();

        // Check if there are some items with the same MD5
        let mut duplicates: Vec<&FileMD5Info> = vec![];
        while let Some(&next_item) = file_iter.peek() {
            if item.md5 == next_item.md5 {
                duplicates.push(next_item);
                file_iter.next();
            } else {
                break;
            }
        }

        // If yes, please print them
        if !duplicates.is_empty() {
            println!("The file {:?} has the following duplicates:", item.path);

            for dupla in duplicates {
                println!("- {:?}", dupla.path);
            }

            println!();
        }
    }
}
