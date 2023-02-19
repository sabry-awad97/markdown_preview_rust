use std::{
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
};

#[derive(Debug)]
pub enum HtmlFileError {
    IOError(io::Error),
}

impl From<io::Error> for HtmlFileError {
    fn from(error: io::Error) -> Self {
        HtmlFileError::IOError(error)
    }
}

pub struct HtmlFile {
    path: PathBuf,
}

impl HtmlFile {
    pub fn new(path: PathBuf) -> Self {
        HtmlFile { path }
    }

    pub fn write(&self, html: &str) -> Result<(), HtmlFileError> {
        let mut file = File::create(&self.path)?;
        write!(file, "{}", html)?;
        Ok(())
    }

    pub fn remove(&self) -> Result<(), HtmlFileError> {
        fs::remove_file(&self.path)?;
        Ok(())
    }
}
