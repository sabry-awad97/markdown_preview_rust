use pulldown_cmark::{html, Options, Parser};
use std::io::Write;
use std::process::Command;
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


    let template = format!(
        r#"<html><head><title>"Markdown Preview"</title></head><body>{body}</body></html>"#,
        body = &html_output
    );

    let mut file = fs::File::create("preview.html").expect("Failed to create temporary file");
    write!(file, "{}", template).expect("Failed to write to temporary file");

    Command::new("cmd")
        .arg("/c")
        .arg("start")
        .arg("preview.html")
        .spawn()
        .expect("Failed to open default browser");
}
