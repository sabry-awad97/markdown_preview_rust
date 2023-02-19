use std::io;
use pulldown_cmark::{html, Options, Parser};

#[derive(Debug)]
pub enum MarkdownError {
    IOError(io::Error),
}

impl From<io::Error> for MarkdownError {
    fn from(error: io::Error) -> Self {
        MarkdownError::IOError(error)
    }
}

pub struct Markdown {
    input: String,
}

impl Markdown {
    pub fn new(input: String) -> Self {
        Markdown { input }
    }

    pub fn to_html(&self) -> String {
        let parser = Parser::new_ext(&self.input, Options::all());
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    }
}
