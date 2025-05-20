use super::StandardError;

use crate::display::{EMPTY_STR, EXPECTED, GOT, write};

use std::fmt::{Debug, Display};

impl Display for StandardError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let padding = write::padding(self.source.as_ref(), None);

		write::error(f, &self.error)?;
		w_expected(f, &padding, &self.expected)?;
		w_got(f, &padding, &self.got)?;
		if let Some(help) = &self.help {
			write::help(f, &padding, Some(help))?;
		}
		write::source_file(f, &padding, self.source_file.as_deref())?;
		write::source(f, self.source.as_ref())?;

		Ok(())
	}
}

impl Debug for StandardError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "\x1b[2K\r{self}")
	}
}

fn w_expected(f: &mut std::fmt::Formatter<'_>, padding: &str, expected: &str) -> std::fmt::Result {
	write!(
		f,
		"\n{padding}{EXPECTED}{expected}\x1b[0m",
		expected = if expected.is_empty() {
			EMPTY_STR
		} else {
			expected
		},
	)
}

fn w_got(f: &mut std::fmt::Formatter<'_>, padding: &str, got: &str) -> std::fmt::Result {
	write!(
		f,
		"\n{padding}{GOT}{got}\x1b[0m",
		got = if got.is_empty() { EMPTY_STR } else { got },
	)
}
