use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let input_path = args.input;

    println!("{:?}", input_path)
}
