use super::SpofedFile;

use ansi::abbrev::{B, D, R, Y};
use hmerr::ParseFileError;

use std::fmt::Debug;
use std::str::FromStr;

impl SpofedFile {
    pub fn parse<T>(&self, key: &str) -> Option<Result<Vec<Vec<T>>, ParseFileError>>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        self.data.get(key).map(|fl| {
            fl.parse::<T>().map_err(|(l, e)| {
                ParseFileError::new(
                    format!(
                        "could not parse token of {B}{Y}{key}{D} for type {B}{R}{}{D}\n{R}{e:?}{D}",
                        std::any::type_name::<T>()
                    ),
                    self.path.to_string_lossy().to_string(),
                    Some(l),
                    self.rule.get(key).unwrap().help(),
                    Some(file!().to_string()),
                    None,
                )
            })
        })
    }
}
