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

#[derive(Debug, Clone, PartialEq)]
pub struct FoundLine(pub Vec<ParsedLine>);

impl FoundLine {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, pl: impl Into<ParsedLine>) {
        self.0.push(pl.into())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn take_once(self) -> ParsedLine {
        self.0.into_iter().next().expect("FoundLine is empty")
    }

    pub fn take_first_token(self) -> String {
        self.0
            .into_iter()
            .next()
            .expect("FoundLine is empty")
            .take(0)
    }
}
