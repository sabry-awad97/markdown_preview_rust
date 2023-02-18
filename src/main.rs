use pulldown_cmark::{html, Options, Parser};
use std::{fs, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let input_path = args.input;

    let input = fs::read_to_string(input_path).expect("Failed to read input file");
    let parser = Parser::new_ext(&input, Options::all());

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    println!("{}", html_output)
}
