use crate::display::{
    idx_padding, w_file, ERROR, FILE_SIGN, HELP, HELP_SIGN, SIDE, SIDE_PADDING_SIGN, SIDE_SIGN,
};
use std::fmt::Display;

#[derive(Debug, Clone, Default)]
pub struct ParseFileError {
    pub error: String,
    pub help: Option<String>,
    pub file: Option<String>,
    pub line: Option<Line>,
    pub source_file: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Line {
    pub line: String,
    pub index: Option<usize>,
    pub wrong: Vec<Wrong>,
}

#[derive(Debug, Clone)]
pub enum Wrong {
    Bit((usize, usize)),
    Str(String),
}

// impl String.into() -> Wrong
// impl (usize, usize).into() -> Wrong

/*
impl Display for ParseFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let padding = idx_padding(self.index);

        writeln!(f, "{ERROR}{}", self.error)?;
        w_file(f, &padding, &self.file, &self.index)?;
        Ok(())
    }
}
*/

impl ParseFileError {
    pub fn new(
        error: impl Into<String>,
        file: impl Into<Option<String>>,
        line: Option<Line>,
        help: impl Into<Option<String>>,
        source_file: impl Into<Option<String>>,
    ) -> Self {
        Self {
            error: error.into(),
            file: file.into(),
            line,
            help: help.into(),
            source_file: source_file.into(),
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

#[macro_export]
macro_rules! pfe {
    ($error:expr $(, h:$help:expr)? $(, f:$file:expr)? $(, l:$line:expr)?) => {
		$crate::parse::ParseFileError {
			error: $error.into(),
			$(file: Some($file.into()),)?
			$(line: Some($line),)?
			$(help: Some($help.into()),)?
			source_file: Some(file!().to_string()),
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
	// pwe!((42, 3)) => Vec<Wrong::Bit((42, 3))>
	// pwe!((42, 3), (84, 7)) => Vec<Wrong::Bit((42, 3)), Wrong::Bit((84, 7))>
	// pwe!("some string") => Vec<Wrong::Str("some string")>
	// pwe!("some string", "another string") => Vec<Wrong::Str("some string"), Wrong::Str("another string")>
	// pwe!("some string", (42, 3)) => Vec<Wrong::Str("some string"), Wrong::Bit((42, 3))>
	($($wrong:expr),*) => {
		vec![$(
			match $wrong {
				($start:expr, $end:expr) => $crate::parse::Wrong::Bit(($start, $end)),
				_ => $crate::parse::Wrong::Str($wrong.into()),
			}
		),*]
	};
}
