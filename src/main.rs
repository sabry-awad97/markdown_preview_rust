use pulldown_cmark::{html, Options, Parser};
use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};
use structopt::StructOpt;
use tempfile::NamedTempFile;

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

    let mut file = NamedTempFile::new()?;
    let path = file.path().with_extension("html");
    write!(file, "{}", template)?;

    Command::new("cmd")
        .args(&["/C", "start", path.to_str().unwrap()])
        .stdout(Stdio::null())
        .spawn()?;

    file.close()?;
    fs::remove_file(path)?;
    Ok(())
}
