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
    let file_size_pairs = scanner::files_to_file_size_tuples(all_files);

    for item in file_size_pairs {
        println!("{:?}: {}", item.0, item.1);
    }
}
