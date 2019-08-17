use structopt::StructOpt;

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
}
