mod display;

use std::error::Error;

pub struct StandardError {
    pub error: String,
    pub expected: String,
    pub got: String,
    pub help: Option<String>,
    pub source_file: Option<String>,
    pub source: Option<Box<dyn Error + Send + Sync>>,
}

impl Error for StandardError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| &**e as _)
    }
}

/// StandardError macro
///
/// # Arguments
///
/// * `error` - error message
/// * `expected` - expected
/// * `got` - got
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
/// use hmerr::{Result, se};
///
/// fn some_main() -> Result<()> {
///     let n = match env::args().nth(1) {
///         None => {
///             return Err(se!(
///                 "no argument given",         // error message
///                 "one argument",              // expected
///                 "",                          // got
///                 h:"please provide a number", // help message
///                                              // no source error (this error was not caused by another one)
///            ))?;
///        }
///        Some(n) => n.parse::<i32>().map_err(
///            |e| {
///                se!(
///                    "could not parse number",       // error message
///                    "i32",                          // expected
///                    n,                              // got
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
macro_rules! se {
	(@some_or_none) => { None };
	(@some_or_none $entity:expr) => { Some($entity) };
    ($error:expr, $expected:expr, $got:expr $(, h:$help:expr)? $(, s:$source:expr)?$(,)?) => {
		$crate::standard::StandardError {
			error: $error.into(),
			expected: $expected.into(),
			got: $got.into(),
			help: se!(@some_or_none $($help.into())?),
			source_file: Some(file!().to_string()),
			source: se!(@some_or_none $(Box::new($source))?),
		}
    };
}
