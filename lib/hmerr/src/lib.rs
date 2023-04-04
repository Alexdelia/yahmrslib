mod parse;
pub use parse::ParseFileError;
pub mod display;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
