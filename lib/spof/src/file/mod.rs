mod file_data;
pub use file_data::{FileData, FileDataKey};
mod key_data;
pub use key_data::KeyData;
mod new;

use std::path::PathBuf;

pub struct SpofedFile<K: FileDataKey> {
    pub path: PathBuf,
    data: FileData<K>,
}

impl<K: FileDataKey> SpofedFile<K> {
    pub fn name(&self) -> String {
        self.path.to_string_lossy().to_string()
    }
}
