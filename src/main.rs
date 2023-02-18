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

#[derive(Debug)]
enum MarkdownError {
    IOError(io::Error),
}

impl From<io::Error> for MarkdownError {
    fn from(error: io::Error) -> Self {
        MarkdownError::IOError(error)
    }
}

struct Markdown {
    input: String,
}

impl Markdown {
    fn new(input: String) -> Self {
        Markdown { input }
    }

    fn to_html(&self) -> String {
        let parser = Parser::new_ext(&self.input, Options::all());
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    }
}

struct HtmlFile {
    path: PathBuf,
}

impl HtmlFile {
    fn new(path: PathBuf) -> Self {
        HtmlFile { path }
    }

    fn write(&self, html: &str) -> Result<(), MarkdownError> {
        let mut file = File::create(&self.path)?;
        write!(file, "{}", html)?;
        Ok(())
    }

    fn remove(&self) -> Result<(), MarkdownError> {
        fs::remove_file(&self.path)?;
        Ok(())
    }
}

struct Preview {
    path: PathBuf,
}

impl Preview {
    fn new(path: PathBuf) -> Self {
        Preview { path }
    }

    fn open(&self) -> Result<(), MarkdownError> {
        let command = if cfg!(windows) {
            "cmd"
        } else if cfg!(unix) || cfg!(macos) {
            "open"
        } else {
            return Err(MarkdownError::IOError(io::Error::new(
                io::ErrorKind::Other,
                "Unsupported platform",
            )));
        };

        process::Command::new(command)
            .args(&["/C", self.path.to_str().unwrap()])
            .stdout(Stdio::null())
            .spawn()?;

        Ok(())
    }
}

struct MarkdownPreview {
    markdown: Markdown,
    html_file: HtmlFile,
    preview: Preview,
}

impl MarkdownPreview {
    fn new(input: String) -> Result<Self, MarkdownError> {
        let markdown = Markdown::new(input);
        let path = Path::new("preview.html");
        let html_file = HtmlFile::new(path.to_path_buf());
        let preview = Preview::new(path.to_path_buf());
        Ok(MarkdownPreview {
            markdown,
            html_file,
            preview,
        })
    }

    fn run(&self) -> Result<(), MarkdownError> {
        let html_output = self.markdown.to_html();
        self.html_file.write(&html_output)?;
        self.preview.open()?;
        thread::sleep(Duration::from_secs(1));
        self.html_file.remove()?;
        Ok(())
    }
}

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let input = fs::read_to_string(&args.input).expect("Failed to read input file");
    let preview = MarkdownPreview::new(input).expect("Failed to initialize Markdown preview");
    if let Err(e) = preview.run() {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}
