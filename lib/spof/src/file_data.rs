use std::fs::File;
use std::io::Result;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct FileData {
    pub name: String,
    pub content: Vec<String>,
    pub diluted: Vec<String>,
}

impl FileData {
    pub fn new(path: impl AsRef<Path>, comment: Option<&'static str>) -> Result<Self> {
        let path = path.as_ref();
        let name = path.to_string_lossy().to_string();
        let content = Self::unwrap_reader(BufReader::new(File::open(path)?))?;
        let diluted = Self::pre_parse(content.clone(), comment);

        Ok(Self {
            name,
            content,
            diluted,
        })
    }

    fn unwrap_reader(reader: BufReader<File>) -> Result<Vec<String>> {
        let mut ret = Vec::new();

        for line in reader.lines() {
            ret.push(line?);
        }

        Ok(ret)
    }

    fn pre_parse(content: Vec<String>, comment: Option<&'static str>) -> Vec<String> {
        let mut ret = Vec::new();

        // remove comment and trim whitespace
        for line in content {
            let mut line = line;

            if let Some(comment) = comment {
                if let Some(i) = line.find(comment) {
                    line = line[..i].to_owned();
                }
            }

            ret.push(line.trim().to_string());
        }

        ret
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
