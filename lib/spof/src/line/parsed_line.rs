use hmerr::parse::{Line, Wrong};

/// ```text
/// (
///    Vec<String>, // the tokens of the line
///    usize        // the line index in the file
/// )
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedLine(pub Vec<String>, pub usize);

impl ParsedLine {
    pub fn new(token: Vec<String>, line_index: usize) -> Self {
        Self(token, line_index)
    }

    pub fn get(&self, index: usize) -> Option<&String> {
        self.0.get(index)
    }

    pub fn take(self, index: usize) -> String {
        self.0
            .into_iter()
            .nth(index)
            .expect("token index out of range")
    }
}

impl From<(Vec<String>, usize)> for ParsedLine {
    fn from((token, line_index): (Vec<String>, usize)) -> Self {
        Self::new(token, line_index)
    }
}

impl From<ParsedLine> for Line {
    fn from(val: ParsedLine) -> Self {
        let s = val.0.join(" ");
        let l = s.len();
        Line::new(s, Some(val.1), vec![Wrong::Bit((0, l))])
    }
}
