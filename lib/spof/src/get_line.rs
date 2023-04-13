use crate::{ExpectedLine, FileData};

use ansi::abbrev::{B, D, Y};
use hmerr::{pfe, Result};

type ParsedLine = (Vec<String>, usize); // (tokens, line_index)

pub fn get_line(f: &FileData, el: ExpectedLine) -> Result<Vec<ParsedLine>> {
    let mut ret: Vec<ParsedLine> = Vec::new();

    for i in 0..f.diluted.len() {
        if f.diluted[i].starts_with(&el.k.keyword) {
            ret.push((el.check(f, i)?, i));
        }
    }

    match el.occurence.check(ret.len()) {
        Ok(_) => Ok(ret),
        Err(e) => pfe!(
            format!("{B}{Y}{}{D} {e}", el.k.keyword),
            h:el.help(),
            f:f.name.clone(),
        )?,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{ExpectedSize, Format, Keyword, Occurence};

    #[test]
    fn test_get_line() {
        let el = ExpectedLine::new(
            Keyword::new("keyword", "desc"),
            Format::new("format", ExpectedSize::Fixed),
            Occurence::Once,
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
        assert!(get_line(&f, el).is_err());

        let el = ExpectedLine::new(
            Keyword::new("keyword", "desc"),
            Format::new("format", ExpectedSize::Fixed),
            Occurence::Once,
        );
        let content = vec!["keyword format".to_string(), "keyword format".to_string()];
        let f = FileData {
            name: "file name".to_string(),
            content: content.clone(),
            diluted: content,
        };
        assert!(get_line(&f, el).is_err());

        let el = ExpectedLine::new(
            Keyword::new("keyword", "desc"),
            Format::new("format", ExpectedSize::Fixed),
            Occurence::OneOrMore,
        );
        let content = vec!["keyword format".to_string(), "keyword format".to_string()];
        let f = FileData {
            name: "file name".to_string(),
            content: content.clone(),
            diluted: content,
        };
        let ret = get_line(&f, el);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap().len(), 2);
    }
}
