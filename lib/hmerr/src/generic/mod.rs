mod display;

use std::error::Error;

pub struct GenericError {
    pub error: String,
    pub help: Option<String>,
    pub source_file: Option<String>,
    pub source: Option<Box<dyn Error + Send + Sync>>,
}

impl Error for GenericError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| &**e as _)
    }
}

/// GenericError macro
///
/// # Arguments
///
/// * `error` - error message
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
/// use std::env;
///
/// use hmerr::{Result, ge};
///
/// fn some_main() -> Result<()> {
///        let n = match env::args().nth(1) {
///        None => {
///            return Err(ge!(
///                "no argument given",          // error message
///                 h:"please provide a number", // help message
///                                              // no source error (this error was not caused by another one)
///            ))?;
///        }
///        Some(n) => n.parse::<i32>().map_err(
///            |e| {
///                ge!(
///                    "could not parse number",       // error message
///                    h:"please provide a valid i32", // help message
///                    s:e,                            // source error
///                )
///            }
///        )?,
///     };
///
///     // do something with n
///
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! ge {
	(@some_or_none) => { None };
	(@some_or_none $entity:expr) => { Some($entity) };
    ($error:expr $(, h:$help:expr)? $(, s:$source:expr)?$(,)?) => {
		$crate::generic::GenericError {
			error: $error.into(),
			help: ge!(@some_or_none $($help.into())?),
			source_file: Some(file!().to_string()),
			source: ge!(@some_or_none $(Box::new($source))?),
		}
    };
}
