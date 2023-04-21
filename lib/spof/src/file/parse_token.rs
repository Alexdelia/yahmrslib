use super::SpofedFile;

use std::str::FromStr;

impl SpofedFile {
    pub fn parse<T>(&self, key: &str) -> Option<Result<Vec<Vec<T>>, T::Err>>
    where
        T: FromStr,
    {
        self.data.get(key).map(|fl| fl.parse::<T>())
    }
}
