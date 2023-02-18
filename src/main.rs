use pulldown_cmark::{html, Options, Parser};
use std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
    process::{self, Stdio},
    thread,
    time::Duration,
};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

#[derive(Debug)]
enum MarkdownError {
    IOError(io::Error),
}

impl From<io::Error> for MarkdownError {
    fn from(error: io::Error) -> Self {
        MarkdownError::IOError(error)
    }
}

struct MarkdownParser {
    input: String,
}

impl MarkdownParser {
    fn new(input: String) -> Self {
        MarkdownParser { input }
    }

    fn parse(&self) -> Result<String, MarkdownError> {
        let parser = Parser::new_ext(&self.input, Options::all());
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        Ok(html_output)
    }
}

struct HtmlTemplate {
    body: String,
}

impl HtmlTemplate {
    fn new(body: String) -> Self {
        HtmlTemplate { body }
    }

    fn generate(&self) -> String {
        format!(
            r#"<html><head><title>"Markdown Preview"</title></head><body>{body}</body></html>"#,
            body = &self.body
        )
    }
}

struct Preview {
    path: PathBuf,
}

impl Preview {
    fn new(path: PathBuf) -> Self {
        Preview { path }
    }

    fn open(&self) -> io::Result<()> {
        let command = if cfg!(windows) {
            "cmd"
        } else if cfg!(unix) || cfg!(macos) {
            "open"
        } else {
            return Err(io::Error::new(io::ErrorKind::Other, "Unsupported platform"));
        };

        process::Command::new(command)
            .args(&["/C", self.path.to_str().unwrap()])
            .stdout(Stdio::null())
            .spawn()?;

        Ok(())
    }
}

fn write_html_to_file(html: &str, path: &Path) -> Result<(), MarkdownError> {
    let mut file = File::create(path)?;
    write!(file, "{}", html)?;
    Ok(())
}

fn run(args: Cli) -> Result<(), MarkdownError> {
    let input = fs::read_to_string(&args.input)?;
    let parser = MarkdownParser::new(input);
    let html_output = parser.parse()?;
    let template = HtmlTemplate::new(html_output);
    let html = template.generate();

    let path = Path::new("preview.html");
    write_html_to_file(&html, path)?;

    let preview = Preview::new(path.to_path_buf());

    preview.open()?;
    thread::sleep(Duration::from_secs(1));
    fs::remove_file(path)?;
    Ok(())
}

fn get_args() -> Result<Cli, MarkdownError> {
    Ok(Cli::from_args())
}

fn main() {
    if let Err(e) = get_args().and_then(run) {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}
