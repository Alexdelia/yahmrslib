#[derive(Debug)]
pub struct Keyword {
    pub keyword: String,
    pub desc: String,
}

impl Keyword {
    pub fn new(keyword: impl Into<String>, desc: impl Into<String>) -> Self {
        Self {
            keyword: keyword.into(),
            desc: desc.into(),
        }
    }
}
