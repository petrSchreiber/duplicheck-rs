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
    let file_md5_pairs = scanner::file_size_info_to_file_md5_info(&file_size_pairs);

    for item in file_md5_pairs {
        println!("{:?}: {:?}", item.path, item.md5);
    }
}
