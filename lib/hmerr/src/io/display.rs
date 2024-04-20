use super::IoError;

use crate::display::write;

use std::fmt::{Debug, Display};
use std::io;

fn display_error(error: &io::Error, file: &str) -> String {
	// TODO: handle more error
	match error.kind() {
		io::ErrorKind::NotFound => format!("\x1b[1;35m{file}\x1b[1;31mnot found\x1b[0m"),
		_ => format!("{kind} {error}", kind=error.kind()),
	}
}

impl Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let padding = write::padding(self.source.as_ref(), None);

        write::error(f, &display_error(&self.error, &self.file))?;
		if let Some(help) = &self.help {
			write::help(f, &padding, Some(help))?;
		}
        write::source_file(f, &padding, self.source_file.as_deref())?;
        write::source(f, self.source.as_ref())?;

        Ok(())
    }
}

impl Debug for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[2K\r{self}")
    }
}
