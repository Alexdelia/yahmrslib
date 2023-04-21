mod new;
mod parse_token;

use crate::{FoundLine, Rule};

use std::collections::HashMap;
use std::path::PathBuf;

pub type FileData = HashMap<String, FoundLine>;

pub struct SpofedFile {
    pub path: PathBuf,
    pub data: FileData,
    pub rule: Rule,
}

impl SpofedFile {
    pub fn get(&self, key: &str) -> Option<&FoundLine> {
        self.data.get(key)
    }
}
