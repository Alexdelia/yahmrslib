use super::{Line, ParseFileError, Wrong};

use crate::display::{
    ERROR, FILE_SIGN, FILE_SOURCE, HELP, HELP_SIGN, LINT_COLOR, LINT_SIGN, SIDE, SIDE_PADDING_SIGN,
    SIDE_SIGN, SOURCE,
};

use std::fmt::{Debug, Display};
use std::ops::Range;

impl Display for ParseFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let index = self.line.as_ref().and_then(|l| l.index);
        let padding = idx_padding(index);

        writeln!(f, "{ERROR}{}", self.error)?;
        if !(self.line.is_none() && self.file.is_none()) {
            w_file(f, &padding, &self.file, &index)?;
        }
        w_line(f, &padding, &self.line)?;
        if !(self.line.is_none() && self.file.is_none() && self.help.is_none()) {
            w_help(f, &padding, &self.help)?;
        }
        w_source_file(f, &padding, &self.source_file)?;
        if let Some(source) = &self.source {
            writeln!(f, "{SOURCE}{source}")?;
        }
        Ok(())
    }
}

impl Debug for ParseFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[2K\r{self}")
    }
}

fn idx_padding(n: Option<usize>) -> String {
    if let Some(n) = n {
        " ".repeat(n.to_string().len())
    } else {
        String::new()
    }
}

pub fn w_file(
    f: &mut std::fmt::Formatter<'_>,
    padding: &str,
    file: &Option<String>,
    index: &Option<usize>,
) -> std::fmt::Result {
    let Some(file) = file else {
		return writeln!(f, "{padding}{FILE_SIGN}");
	};

    let index: String = if let Some(index) = index {
        format!(":{}", index)
    } else {
        String::new()
    };

    writeln!(f, "{padding}{FILE_SIGN}{file}{index}")
}

pub fn w_line(
    f: &mut std::fmt::Formatter<'_>,
    padding: &str,
    line: &Option<Line>,
) -> std::fmt::Result {
    let Some(line) = line else {
		return Ok(());
	};

    writeln!(f, "{padding}{SIDE_PADDING_SIGN}")?;

    if let Some(index) = line.index {
        writeln!(f, "{SIDE}{index}{SIDE_SIGN}{}", line.line)?;
    } else {
        writeln!(f, "{padding}{SIDE_SIGN}{}", line.line)?;
    }

    w_lint(f, padding, &line.line, &line.wrong)?;

    writeln!(f, "{padding}{SIDE_PADDING_SIGN}")
}

fn w_lint(
    f: &mut std::fmt::Formatter<'_>,
    padding: &str,
    line: &str,
    wrong: &Vec<Wrong>,
) -> std::fmt::Result {
    let r = construct_range(wrong, line);
    if r.is_empty() {
        return Ok(());
    }

    let mut s = String::with_capacity(r.last().unwrap().end);
    let mut start = 0;
    let lint_sign = LINT_SIGN.to_string();
    for i in r {
        s.push_str(&" ".repeat(i.start - start));
        s.push_str(&lint_sign.repeat(i.end - i.start));
        start = i.end;
    }
    writeln!(f, "{padding}{SIDE_SIGN}{LINT_COLOR}{s}\x1b[0m")?;

    Ok(())
}

fn construct_range(wrong: &Vec<Wrong>, line: &str) -> Vec<Range<usize>> {
    let mut r: Vec<Range<usize>> = Vec::new();
    for w in wrong {
        match w {
            Wrong::Bit((start, end)) => {
                r.push(*start..*start + *end);
            }
            Wrong::Str(s) => {
                let mut start = 0;
                while let Some(i) = line[start..].find(s) {
                    start += i;
                    r.push(start..start + s.len());
                    start += s.len();
                }
            }
        }
    }

    simplifly(&mut r);
    r
}

fn simplifly(r: &mut Vec<Range<usize>>) {
    r.sort_by(|a, b| a.start.cmp(&b.start));
    let mut i = 0;
    while i < r.len() - 1 {
        if r[i].end >= r[i + 1].start {
            r[i].end = r[i + 1].end;
            r.remove(i + 1);
        } else {
            i += 1;
        }
    }
}

pub fn w_help(
    f: &mut std::fmt::Formatter<'_>,
    padding: &str,
    help: &Option<String>,
) -> std::fmt::Result {
    if let Some(help) = help {
        writeln!(f, "{padding}{HELP_SIGN}{HELP}{help}")
    } else {
        writeln!(f, "{padding}{HELP_SIGN}")
    }
}

pub fn w_source_file(
    f: &mut std::fmt::Formatter<'_>,
    padding: &str,
    source_file: &Option<String>,
) -> std::fmt::Result {
    if let Some(source_file) = source_file {
        writeln!(f, "{padding}{FILE_SOURCE}{source_file}")
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idx_padding() {
        assert_eq!(idx_padding(None), "");
        assert_eq!(idx_padding(Some(0)), " ");
        assert_eq!(idx_padding(Some(1)), " ");
        assert_eq!(idx_padding(Some(21)), "  ");
        assert_eq!(idx_padding(Some(9)), " ");
        assert_eq!(idx_padding(Some(100)), "   ");
    }
}
