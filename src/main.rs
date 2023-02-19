mod cli;
mod markdown;

use cli::cli::Cli;
use markdown::{
    html_file::HtmlFile,
    markdown::{Markdown, MarkdownError},
    preview::Preview,
};
use std::{fs, io, path::Path, thread, time::Duration};
use structopt::StructOpt;

#[derive(Debug)]
enum AppError {
    MarkdownError(MarkdownError),
    IOError(io::Error),
}

impl From<MarkdownError> for AppError {
    fn from(error: MarkdownError) -> Self {
        AppError::MarkdownError(error)
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::IOError(error)
    }
}

struct MarkdownPreview {
    markdowns: Vec<Markdown>,
    html_file: HtmlFile,
    preview: Preview,
}

impl MarkdownPreview {
    fn new(inputs: Vec<String>) -> Result<Self, AppError> {
        let markdowns = inputs.into_iter().map(Markdown::new).collect();
        let path = Path::new("preview.html");
        let html_file = HtmlFile::new(path.to_path_buf());
        let preview = Preview::new(path.to_path_buf());
        Ok(MarkdownPreview {
            markdowns,
            html_file,
            preview,
        })
    }

    fn run(&self) -> Result<(), AppError> {
        let html_output = self
            .markdowns
            .iter()
            .map(|m| m.to_html())
            .collect::<String>();
        self.html_file.write(&html_output).unwrap();
        self.preview.open().unwrap();
        thread::sleep(Duration::from_secs(1));
        self.html_file.remove().unwrap();
        Ok(())
    }
}

fn main() {
    let args = Cli::from_args();
    let inputs = args
        .inputs
        .into_iter()
        .map(|p| fs::read_to_string(&p).unwrap())
        .collect();
    let preview = MarkdownPreview::new(inputs).expect("Failed to initialize Markdown preview");
    if let Err(e) = preview.run() {
        eprintln!("Error: {:?}", e);
        std::process::exit(1);
    }
}
