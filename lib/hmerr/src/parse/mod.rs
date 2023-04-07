mod display;

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
    Bit((usize, usize)), // if bit from idx 42 to 45, then start = 42, end = 3
    Str(String),
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
	($($wrong:expr),*) => {
		vec![$($wrong.into()),*]
	};
}
