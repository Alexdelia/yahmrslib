mod file;
pub use file::{FileData, FileDataKey, KeyData, SpofedFile};

mod rule;
pub use rule::{
    expected_line::ExpectedLine,
    format::{ExpectedSize, Format},
    keyword::Keyword,
    occurence::Occurence,
};

mod line;
pub use line::{FoundLine, ParsedLine};
