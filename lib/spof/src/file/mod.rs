mod key_data;
mod new;
mod parse_token;
pub use key_data::KeyData;

use crate::{ExpectedLine, FoundLine, Rule};

use std::collections::HashMap;
use std::path::PathBuf;

pub type FileData<S> = [KeyData; S];

pub struct SpofedFile<D> {
    pub path: PathBuf,
    pub data: D,
}

impl<K> SpofedFile<K> {
    pub fn name(&self) -> String {
        self.path.to_string_lossy().to_string()
    }

    pub fn get(&self, key: K) -> &FoundLine {
        self.data.get(key).unwrap().data // should never panic because HashMap is populated with all keys from Rule
    }
}
