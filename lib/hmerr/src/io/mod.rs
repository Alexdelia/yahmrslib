mod display;

use std::error::Error;
use std::io;

pub struct IoError {
    pub file: String,
    pub error: io::Error,
    pub help: Option<String>,
    pub source_file: Option<String>,
    pub source: Option<Box<dyn Error + Send + Sync>>,
}

impl Error for IoError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| &**e as _)
    }
}
