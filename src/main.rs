use pulldown_cmark::{html, Options, Parser};
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    let input_path = &args.input;

    let input = fs::read_to_string(input_path)?;
    let parser = Parser::new_ext(&input, Options::all());

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let template = format!(
        r#"<html><head><title>"Markdown Preview"</title></head><body>{body}</body></html>"#,
        body = &html_output
    );

    let mut file = File::create("preview.html")?;
    write!(file, "{}", template)?;

    Command::new("cmd")
        .args(&["/C", "start", "preview.html"])
        .stdout(Stdio::null())
        .spawn()?;

    Ok(())
}
