use super::SpofedFile;
use crate::{ExpectedLine, FoundLine, ParsedLine, Rule};

use ansi::abbrev::{B, D, G, Y};
use hmerr::{pfe, ple, pwe, Result};

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

impl SpofedFile {
    pub fn new(path: impl Into<PathBuf>, comment: Option<&str>, rule: Rule) -> Result<Self> {
        let path: PathBuf = path.into();
        let name = path.to_string_lossy().to_string();

        let reader = BufReader::new(File::open(&path)?);

        let mut data = HashMap::new();

        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            if let Some((keyword, pl)) = parse(&name, comment, &rule, line, i)? {
                let fl = data.entry(keyword).or_insert_with(FoundLine::new);
                fl.push(pl);
            }
        }

        Ok(Self { path, data })
    }
}

fn parse(
    name: &str,
    comment: Option<&str>,
    rule: &Rule,
    line: String,
    i: usize,
) -> Result<Option<(String, ParsedLine)>> {
    let diluted = pre_parse(line.clone(), comment);
    let mut split: Vec<String> = diluted.split_whitespace().map(|s| s.to_string()).collect();

    if split.is_empty() {
        return Ok(None);
    }

    let keyword = split.remove(0);
    let el = get_expected_line(name, rule, &line, &keyword, i)?;
    el.check(name, line, &split, i)?;

    Ok(Some((keyword, ParsedLine::new(split, i))))
}

/// remove comment and trim whitespace
fn pre_parse(line: String, comment: Option<&str>) -> String {
    let mut line = line;

    if let Some(comment) = comment {
        if let Some(i) = line.find(comment) {
            line = line[..i].to_owned();
        }
    }

    line.trim().to_string()
}

fn get_expected_line<'r>(
    name: &str,
    rule: &'r Rule,
    line: &str,
    keyword: &str,
    i: usize,
) -> Result<&'r ExpectedLine> {
    match rule.get(keyword) {
        Some(el) => Ok(el),
        None => pfe!(
            format!("unsupported keyword {B}{Y}{keyword}{D}"),
            h: format!(
                "no rule for keyword {B}{Y}{keyword}{D}
here is a list of valid keyword:
{keywords}",
                keywords = rule
                    .keywords()
                    .into_iter()
                    .map(|k| format!(
                        "\t- {B}{G}{keyword}{D}: {B}{desc}{D}",
                        keyword = k.keyword,
                        desc = k.desc
                    ))
                    .collect::<Vec<String>>()
                    .join("\n")
            ),
            f: name,
            l: ple!(line, i: i, w: pwe!(keyword))
        )?,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pre_parse_single_char_comment() {
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

        for (c, e) in content.iter().zip(expected.iter()) {
            assert_eq!(pre_parse(c.to_string(), Some("#")), e.to_string());
        }
    }

    #[test]
    fn test_pre_parse_multi_char_comment() {
        let content = vec![
            "v 1.0 2.0 3.0".to_string(),
            "v 4.0 5.0 6.0 // this is a comment".to_string(),
            "v 7.0 8.0 9.0".to_string(),
            "//comment".to_string(),
            "// comment".to_string(),
            "v 1.0 2.0 3.0// comment".to_string(),
            "      // comment".to_string(),
            "    v 1.0 2.0 3.0 // comment     ".to_string(),
            "\tv 1.0 2.0 3.0\t//comment\t".to_string(),
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

        for (c, e) in content.iter().zip(expected.iter()) {
            assert_eq!(pre_parse(c.to_string(), Some("//")), e.to_string());
        }
    }
}
