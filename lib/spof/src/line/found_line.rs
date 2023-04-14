use super::ParsedLine;

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

impl Default for FoundLine {
    fn default() -> Self {
        Self::new()
    }
}
