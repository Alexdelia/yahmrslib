use std::fmt::Display;

const ERROR: &str = "\x1b[0m\x1b[1;31merror\x1b[39m:\x1b[0m\t";
const HELP: &str = "\x1b[0m\x1b[36mhelp\x1b[0m:\t";
const SIDE: &str = "\x1b[0m\x1b[34m";

const FILE_SIGN: &str = "\x1b[0m\x1b[1;34m-->\x1b[0m ";
const SIDE_SIGN: &str = "\x1b[0m \x1b[1;34m|\x1b[0m ";
const HELP_SIGN: &str = "\x1b[0m \x1b[1;34m=\x1b[0m ";

// state
#[derive(Debug, Clone)]
pub struct NoLine;
#[derive(Debug, Clone)]
pub struct Line(String);

#[derive(Debug, Clone)]
pub struct ParseFileError<L> {
    error: String,
    file: Option<String>,
    line: L,
    index: Option<usize>,
    wrong_bit: Option<Vec<(usize, usize)>>,
    help: Option<String>,
    source_file: Option<String>,
}

impl<L> Display for ParseFileError<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let padding: String = if let Some(index) = self.index {
            " ".repeat(index.to_string().len())
        } else {
            String::new()
        };

        writeln!(f, "{ERROR}{}", self.error)?;
        if let Some(file) = &self.file {
            writeln!(
                f,
                "{padding}{FILE_SIGN}{file}{index}",
                index = self
                    .index
                    .map(|i| format!(":{}", i))
                    .unwrap_or(String::new()),
            )?;
        }
        Ok(())
    }
}

impl ParseFileError<NoLine> {
    pub fn new(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            file: None,
            line: NoLine,
            index: None,
            wrong_bit: None,
            help: None,
            source_file: None,
        }
    }
}

impl<L> ParseFileError<L> {
    pub fn line(self, line: impl Into<String>) -> ParseFileError<Line> {
        ParseFileError {
            error: self.error,
            file: self.file,
            line: Line(line.into()),
            index: self.index,
            wrong_bit: self.wrong_bit,
            help: self.help,
            source_file: self.source_file,
        }
    }

    pub fn file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }

    pub fn help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    pub fn source_file(mut self, source_file: impl Into<String>) -> Self {
        self.source_file = Some(source_file.into());
        self
    }
}

impl ParseFileError<Line> {
    pub fn index(mut self, index: usize) -> Self {
        self.index = Some(index);
        self
    }

    pub fn wrong_bit(mut self, wrong_bit: Vec<(usize, usize)>) -> Self {
        self.wrong_bit = Some(wrong_bit);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file_error() {
        dbg!(ParseFileError::new("error")
            .line("line")
            .index(1)
            .wrong_bit(vec![(1, 2)])
            .help("help")
            .source_file(file!()));

        dbg!(ParseFileError::new("error")
            .help("help")
            .line("line")
            .index(1)
            .wrong_bit(vec![(1, 2)]));

        dbg!(ParseFileError::new("error")
            .help("help")
            .line("line")
            .wrong_bit(vec![(1, 2)])
            .index(1));

        /*
        dbg!(ParseFileError::new("error")
            .help("help")
            .wrong_bit(vec![(1, 2)])
            .line("line")
            .index(1));
        */

        /*
        dbg!(ParseFileError::new("error")
            .help("help")
            .index(1)
            .line("line")
            .wrong_bit(vec![(1, 2)]);
        */
    }
}
