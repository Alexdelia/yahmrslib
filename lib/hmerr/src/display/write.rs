use std::fmt::Display;

use crate::display::{
    ERROR, FILE_COLOR, FILE_SIGN, FILE_SOURCE, HELP, HELP_SIGN, LINT_COLOR, LINT_SIGN, SIDE,
    SIDE_PADDING_SIGN, SIDE_SIGN, SOURCE, SOURCE_SIDE_SIGN,
};

pub fn error(f: &mut std::fmt::Formatter<'_>, error: &impl Display) -> std::fmt::Result {
    write!(f, "{ERROR}{error}")
}

pub fn padding<E>(source: &Option<E>, n: Option<usize>) -> String {
    let padding = n_to_padding(n);

    if source.is_some() {
        return SOURCE_SIDE_SIGN.to_string() + &padding;
    }

    padding
}

fn n_to_padding(n: Option<usize>) -> String {
    match n {
        Some(n) => {
            let n = n.to_string();
            let n = if n.len() == 1 { " " } else { "" };
            format!("{n} ")
        }
        None => " ".to_string(),
    }
}

pub fn file(
    f: &mut std::fmt::Formatter<'_>,
    padding: &str,
    file: Option<&str>,
    index: &Option<usize>,
) -> std::fmt::Result {
    let Some(file) = file else {
        return write!(f, "\n{padding}{FILE_SIGN}");
    };

    let index: String = if let Some(index) = index {
        format!("\x1b[0m{SIDE}:\x1b[0m{FILE_COLOR}{index}")
    } else {
        String::new()
    };

    write!(f, "\n{padding}{FILE_SIGN}{FILE_COLOR}{file}{index}\x1b[0m")
}

pub fn help(
    f: &mut std::fmt::Formatter<'_>,
    padding: &str,
    help: &Option<String>,
) -> std::fmt::Result {
    if let Some(help) = help {
        write!(
            f,
            "\n{padding}{HELP_SIGN}{HELP}{help}",
            help = help.replace('\n', format!("\n{padding}\t").as_str())
        )
    } else {
        write!(f, "\n{padding}{HELP_SIGN}")
    }
}

pub fn source_file(
    f: &mut std::fmt::Formatter<'_>,
    padding: &str,
    source_file: Option<&str>,
) -> std::fmt::Result {
    if let Some(source_file) = source_file {
        write!(f, "\n{padding}{FILE_SOURCE}{source_file}\x1b[0m")
    } else {
        Ok(())
    }
}

pub fn source(f: &mut std::fmt::Formatter<'_>, source: &Option<impl Display>) -> std::fmt::Result {
    if let Some(source) = source {
        write!(f, "\n{SOURCE}{source}\x1b[0m")
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_to_padding() {
        assert_eq!(n_to_padding(None), " ");
        assert_eq!(n_to_padding(Some(0)), "  ");
        assert_eq!(n_to_padding(Some(1)), "  ");
        assert_eq!(n_to_padding(Some(21)), "   ");
        assert_eq!(n_to_padding(Some(9)), "  ");
        assert_eq!(n_to_padding(Some(100)), "    ");
    }
}
