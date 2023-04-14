mod parse;
pub use parse::spof;

mod rule;
pub use rule::{
    expected_line::ExpectedLine,
    format::{ExpectedSize, Format},
    keyword::Keyword,
    occurence::Occurence,
    Rule,
};

mod line;
pub use line::{FoundLine, ParsedLine};
