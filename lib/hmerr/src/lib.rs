pub mod display;
pub use display::{ERROR, WARNING};
pub mod generic;
pub use generic::GenericError;
pub mod standard;
pub use standard::StandardError;
pub mod io;
pub use io::IoError;
pub mod parse;
pub use parse::ParseFileError;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
