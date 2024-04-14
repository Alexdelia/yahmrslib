use super::IoError;

use crate::display::{
    write, ERROR, FILE_COLOR, FILE_SIGN, FILE_SOURCE, HELP, HELP_SIGN, LINT_COLOR, LINT_SIGN, SIDE,
    SIDE_PADDING_SIGN, SIDE_SIGN, SOURCE, SOURCE_SIDE_SIGN,
};

use std::fmt::{Debug, Display};

impl Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let padding = write::padding(&self.source, None);

        write::error(f, &self.error)?;
        write::file(f, &padding, Some(&self.file), &None)?;
        write::help(f, &padding, &self.help)?;
        write::source_file(f, &padding, self.source_file.as_deref())?;
        write::source(f, &self.source)?;

        Ok(())
    }
}

impl Debug for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[2K\r{self}")
    }
}
