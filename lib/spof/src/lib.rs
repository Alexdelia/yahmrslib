mod parse;
pub use parse::spof;

mod file_data;
pub use file_data::FileData;

mod rule;
pub use rule::{
    expected_line::ExpectedLine,
    format::{ExpectedSize, Format},
    keyword::Keyword,
    occurence::Occurence,
    Rule,
};

mod line;
pub use line::{get_line, FoundLine, ParsedLine};
