use super::{Format, Keyword, Occurence};
use crate::FileData;

use yahmrslib::ansi::abbrev::{B, D, G, M, R, Y};
use yahmrslib::hmerr::{pfe, ple, pwe, Result};

pub struct ExpectedLine {
    pub k: Keyword,
    pub format: Format,
    pub occurence: Occurence,
}

impl ExpectedLine {
    pub fn new(k: Keyword, format: Format, occurence: Occurence) -> Self {
        Self {
            k,
            format,
            occurence,
        }
    }

    pub fn check(&self, f: &FileData, line_index: usize) -> Result<Vec<String>> {
        let binding = f.diluted[line_index].replacen(&self.k.keyword, "", 1);
        let line = binding.trim();
        match self.format.check(line) {
            Ok(v) => Ok(v),
            Err((expected, got)) => pfe!(
                format!("expected {B}{G}{expected}{D} token after {B}{Y}{keyword}{D}, got {B}{R}{got}{D}",
                    keyword=self.k.keyword,
                ),
                h:self.help(),
                f:f.name.clone(),
                l:ple!(line, i:line_index, w:pwe!((0, line.len())))
            )?,
        }
    }

    pub fn help(&self) -> String {
        format!(
            "{B}{Y}{keyword}{D} define {desc}
the line must follow the format: `{B}{keyword} {M}{format}{D}`",
            keyword = self.k.keyword,
            desc = self.k.desc,
            format = self.format.token,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ExpectedSize;

    #[test]
    fn test_expected_line() {
        let el = ExpectedLine::new(
            Keyword::new("keyword", "desc"),
            Format::new("format", ExpectedSize::Fixed),
            Occurence::OneOrMore,
        );
        let content = vec![
            "keyword format".to_string(),
            "keyword format wrong".to_string(),
        ];
        let f = FileData {
            name: "file name".to_string(),
            content: content.clone(),
            diluted: content,
        };
        assert!(el.check(&f, 0).is_ok());
        assert!(el.check(&f, 1).is_err());
    }
}
