mod display;

use std::error::Error;

#[derive(Default)]
pub struct ParseFileError {
    pub error: String,
    pub help: Option<String>,
    pub file: Option<String>,
    pub line: Option<Line>,
    pub source_file: Option<String>,
    pub source: Option<Box<dyn Error + Send + Sync>>,
}

#[derive(Debug, Clone, Default)]
pub struct Line {
    pub line: String,
    pub index: Option<usize>,
    pub wrong: Vec<Wrong>,
}

#[derive(Debug, Clone)]
pub enum Wrong {
    Bit((usize, usize)), // if bit from idx 42 to 45, then start = 42, end = 3
    Str(String),
}

impl Error for ParseFileError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| &**e as _)
    }
}

impl From<String> for Wrong {
    fn from(s: String) -> Self {
        Self::Str(s)
    }
}

impl From<&str> for Wrong {
    fn from(s: &str) -> Self {
        Self::Str(s.to_string())
    }
}

impl From<(usize, usize)> for Wrong {
    fn from((start, end): (usize, usize)) -> Self {
        Self::Bit((start, end))
    }
}

impl ParseFileError {
    pub fn new(
        error: impl Into<String>,
        file: impl Into<Option<String>>,
        line: Option<Line>,
        help: impl Into<Option<String>>,
        source_file: impl Into<Option<String>>,
        source: Option<Box<dyn Error + Send + Sync>>,
    ) -> Self {
        Self {
            error: error.into(),
            file: file.into(),
            line,
            help: help.into(),
            source_file: source_file.into(),
            source,
        }
    }
}

impl Line {
    pub fn new(line: impl Into<String>, index: Option<usize>, wrong: Vec<Wrong>) -> Self {
        Self {
            line: line.into(),
            index,
            wrong,
        }
    }
}

/// ParseFileError macro
///
/// # Arguments
///
/// * `error` - error message
/// * `h:help` - help message	[optional]
/// * `f:file` - file name		[optional]
/// * `l:line` - line (can be generated with `ple!`)	[optional]
/// * `s:source` - source error	[optional]
///
/// optional don't need to define (don't event need `None`)
/// but they must be in order (for now)
///
/// # Example
///
/// ```
/// use hmerr::{Result, pfe, ple, pwe};
///
/// const FILE_NAME: &str = "test.txt";
/// const FILE_CONTENT: &str = "John 42
/// Will 21
/// Bob -21";
///
/// fn main() -> Result<()> {
///		for (i, line) in FILE_CONTENT.lines().enumerate() {
/// 		let s = line.split_whitespace().collect::<Vec<_>>();
/// 		if s.len() != 2 {
/// 			Err(pfe!(
/// 				"line should have 2 elements\n<name> <number>",	// error message
/// 																// no help message
/// 				f:FILE_NAME,									// file name
/// 				l:ple!(line, i:i)								// line	(`wrong` is not specified)
/// 																// no source error
/// 			))?;
/// 		}
///
/// 		let name = s[0];
/// 		let number = match s[1].parse::<u32>() {
/// 			Ok(n) => n,
/// 			Err(e) => {
/// 				return Err(pfe!(
/// 					"failed to parse <number>",					// error message
/// 					h:"<number> is supposed to be a `u32`",		// help message
/// 					f:FILE_NAME,								// file name
/// 					l:ple!(line, i:i, w:pwe!(s[1])),			// line	(`wrong` is specified, it will search for `s[1]` in `line` and highlight it)
/// 					s:e											// source error
/// 				))?;
/// 			}
/// 		};
///
/// 		// do something with name and number
///		}
///
/// 	Ok(())
/// }
/// ```
#[macro_export]
macro_rules! pfe {
    ($error:expr $(, h:$help:expr)? $(, f:$file:expr)? $(, l:$line:expr)? $(, s:$source:expr)?) => {
		$crate::parse::ParseFileError {
			error: $error.into(),
			$(file: Some($file.into()),)?
			$(line: Some($line),)?
			$(help: Some($help.into()),)?
			source_file: Some(file!().to_string()),
			$(source: Some(Box::new($source)),)?
			..Default::default()
		}
    };
}

#[macro_export]
macro_rules! ple {
	($line:expr $(, i:$index:expr)? $(, w:$wrong:expr)?) => {
		$crate::parse::Line {
			line: $line.into(),
			$(index: Some($index),)?
			$(wrong: $wrong,)?
			..Default::default()
		}
	};
}

#[macro_export]
macro_rules! pwe {
	($($wrong:expr),*) => {
		vec![$($wrong.into()),*]
	};
}
