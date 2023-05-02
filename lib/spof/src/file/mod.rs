mod file_data;
pub use file_data::{FileData, FileDataKey};
mod key_data;
use hmerr::ParseFileError;
pub use key_data::KeyData;
mod new;

use ansi::abbrev::{B, D, R, Y};

use std::path::PathBuf;
use std::str::FromStr;

pub struct SpofedFile<K: FileDataKey> {
    pub path: PathBuf,
    data: FileData<K>,
}

impl<K: FileDataKey> SpofedFile<K> {
    pub fn name(&self) -> String {
        self.path.to_string_lossy().to_string()
    }

    pub fn parse<T>(&self, k: K) -> Result<Vec<Vec<T>>, Box<ParseFileError>>
    where
        T: FromStr,
    {
        Ok(self[k].data.parse::<T>().map_err(|(l, _)| {
            ParseFileError::new(
                format!(
                    "could not parse {B}{Y}{keyword}{D} as {B}{R}{t}{D}",
                    keyword = self[k].rule.k.keyword,
                    t = std::any::type_name::<T>(),
                ),
                self.name(),
                Some(l),
                self[k].rule.help(),
                Some(file!().to_string()),
                None,
            )
        })?)
    }
}
