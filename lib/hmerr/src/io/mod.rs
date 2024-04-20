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

/// IoError macro
///
/// # Arguments
///
/// * `file` - file name
/// * `error` - io error
/// * `h:help` - help message   |optional|
/// * `s:source` - source error |optional|
///
/// optional don't need to be defined (don't even need `None`)
///
/// but they must be in order (for now)
///
/// # Example
///
/// ```
/// use std::fs::File;
/// use std::io::prelude::*;
/// 
/// use hmerr::{Result, ioe};
///
/// const FILE_PATH: &str = "i_do_not_exist.txt";
///
/// fn some_main() -> Result<()> {
///     let mut file = File::open(FILE_PATH).map_err(
///         |e| {
///             ioe!(
///                 FILE_PATH, // file path
///                 e,         // io::error
///                            // no help message
///                            // no source error (this error was not caused by another one)
///             )
///         }
///     )?;
/// 
///     let mut content = String::new();
///     file.read_to_string(&mut content).map_err(
///         |e| {
///             ioe!(
///                 FILE_PATH,                          // file path
///                 e,                                  // io::error
///                 h:"make sure the file is readable", // optional help message
///                                                     // no source error (this error was not caused by another one)
///             )
///         }
///     )?;
/// 
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! ioe {
    ($file:expr, $error:expr $(, h:$help:expr)? $(, s:$source:expr)?$(,)?) => {
		Err(#[allow(clippy::needless_update)] $crate::parse::IoError {
			file: $file.into()
			error: $error.into(),
			$(help: Some($help.into()),)?
			source_file: Some(file!().to_string()),
			$(source: Some(Box::new($source)),)?
			..Default::default()
		})
    };
}
