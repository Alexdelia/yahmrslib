use crate::{FoundLine, Rule};

use ansi::abbrev::{B, D, G, Y};
use hmerr::{pfe, ple, pwe, Result};

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn spof(
    path: impl AsRef<Path>,
    comment: Option<&str>,
    rule: Rule,
) -> Result<HashMap<String, FoundLine>> {
    let path = path.as_ref();
    let name = path.to_string_lossy().to_string();

    let reader = BufReader::new(File::open(path)?);

    let mut ret = HashMap::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        let diluted = pre_parse(line, comment);
    }

    // read
    // parse each line
    //  preparse line
    //	check rule
    // if rule is met, add to ret

    Ok(ret)
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

fn check(
    name: String,
    rule: Rule,
    line: String,
    diluted: String,
    i: usize,
) -> Result<Option<FoundLine>> {
    let mut split: Vec<String> = diluted.split_whitespace().map(|s| s.to_string()).collect();

    if split.is_empty() {
        return Ok(None);
    }

    let keyword = split.remove(0);
    let Some(el) = rule.get(&keyword) else {
		return pfe!(
			format!("unsupported keyword {B}{Y}{keyword}{D}"),
			h:format!(
"no rule for keyword {B}{Y}{keyword}{D}
here is a list of valid keyword:
{keywords}",
				keywords=rule
					.keywords()
					.into_iter()
					.map(|k| format!(
						"\t- {B}{G}{keyword}{D}: {B}{desc}{D}",
						keyword=k.keyword,
						desc=k.desc
					))
					.collect::<Vec<String>>()
					.join("\n")
			),
			f:name,
			l:ple!(line, i:i, w:pwe!(keyword))
		)?
	};
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
