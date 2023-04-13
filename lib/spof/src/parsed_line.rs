use crate::Occurence;

/// ```text
/// (
///    Vec<String>, // the tokens of the line
///    usize        // the line index in the file
/// )
/// ```
pub struct ParsedLine(Vec<String>, usize);

impl ParsedLine {
    pub fn new(tokens: Vec<String>, line_index: usize) -> Self {
        Self(tokens, line_index)
    }
}
