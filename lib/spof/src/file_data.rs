use crate::{FoundLine, Rule};

use std::collections::HashMap;
use std::fs::File;
use std::io::Result;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct FileData {
    name: String,
    content: Vec<String>,
    comment: Option<String>,
    rule: Rule,
    found: HashMap<String, FoundLine>,
}

impl FileData {
    pub fn new(
        path: impl AsRef<Path>,
        comment: impl Into<Option<String>>,
        rule: Rule,
    ) -> Result<Self> {
        let path = path.as_ref();
        let name = path.to_string_lossy().to_string();
        let content = Self::unwrap_reader(BufReader::new(File::open(path)?))?;

        Ok(Self {
            name,
            content,
            comment: comment.into(),
            rule,
            found: HashMap::new(),
        })
    }

    fn unwrap_reader(reader: BufReader<File>) -> Result<Vec<String>> {
        let mut ret = Vec::new();

        for line in reader.lines() {
            ret.push(line?);
        }

        Ok(ret)
    }

    /// remove comment and trim whitespace
    fn pre_parse(&self, line: String) -> String {
        let mut line = line;

        if let Some(comment) = &self.comment {
            if let Some(i) = line.find(comment) {
                line = line[..i].to_owned();
            }
        }

        line.trim().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pre_parse() {
        let content = vec![
            "v 1.0 2.0 3.0".to_string(),
            "v 4.0 5.0 6.0 # this is a comment".to_string(),
            "v 7.0 8.0 9.0".to_string(),
            "#comment".to_string(),
            "# comment".to_string(),
            "v 1.0 2.0 3.0# comment".to_string(),
            "      # comment".to_string(),
            "    v 1.0 2.0 3.0 # comment     ".to_string(),
            "\tv 1.0 2.0 3.0\t#comment\t".to_string(),
        ];

        let expected = vec![
            "v 1.0 2.0 3.0".to_string(),
            "v 4.0 5.0 6.0".to_string(),
            "v 7.0 8.0 9.0".to_string(),
            "".to_string(),
            "".to_string(),
            "v 1.0 2.0 3.0".to_string(),
            "".to_string(),
            "v 1.0 2.0 3.0".to_string(),
            "v 1.0 2.0 3.0".to_string(),
        ];

        assert_eq!(FileData::pre_parse(content, Some("#")), expected);
    }
}
