use std::{
    io,
    process::{self, Stdio},
    path::PathBuf,
};

#[derive(Debug)]
pub enum PreviewError {
    IOError(io::Error),
    ProcessError(io::Error),
    UnsupportedPlatform,
}

impl From<io::Error> for PreviewError {
    fn from(error: io::Error) -> Self {
        PreviewError::IOError(error)
    }
}

pub struct Preview {
    path: PathBuf,
}

impl Preview {
    pub fn new(path: PathBuf) -> Self {
        Preview { path }
    }

    pub fn open(&self) -> Result<(), PreviewError> {
        let command = if cfg!(windows) {
            "cmd"
        } else if cfg!(unix) || cfg!(macos) {
            "open"
        } else {
            return Err(PreviewError::UnsupportedPlatform);
        };

        process::Command::new(command)
            .args(&["/C", self.path.to_str().unwrap()])
            .stdout(Stdio::null())
            .spawn()
            .map_err(|e| PreviewError::ProcessError(e))?;

        Ok(())
    }
}
