pub mod parse;
pub use parse::ParseFileError;
mod display;
pub use display::{ERROR, WARNING};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
