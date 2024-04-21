use super::GenericError;

use crate::display::write;

use std::fmt::{Debug, Display};

impl Display for GenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let padding = write::padding(self.source.as_ref(), None);

        write::error(f, &self.error)?;
        if let Some(help) = &self.help {
            write::help(f, &padding, Some(help))?;
        }
        write::source_file(f, &padding, self.source_file.as_deref())?;
        write::source(f, self.source.as_ref())?;

        Ok(())
    }
}

impl Debug for GenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[2K\r{self}")
    }
}
