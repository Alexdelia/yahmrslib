pub const ERROR: &str = "\x1b[0m\x1b[1;31merror\x1b[39m:\x1b[0m\t";
pub const HELP: &str = "\x1b[0m\x1b[36mhelp\x1b[0m:\t";
pub const SIDE: &str = "\x1b[0m\x1b[34m";

pub const FILE_SIGN: &str = "\x1b[0m\x1b[1;34m╭╴◊\x1b[0m ";
pub const SIDE_PADDING_SIGN: &str = "\x1b[0m\x1b[1;34m┆\x1b[0m ";
pub const SIDE_SIGN: &str = "\x1b[0m\x1b[1;34m│\x1b[0m ";
pub const HELP_SIGN: &str = "\x1b[0m\x1b[1;34m╧\x1b[0m ";

pub fn idx_padding(n: Option<usize>) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idx_padding() {
        assert_eq!(idx_padding(None), "");
        assert_eq!(idx_padding(Some(0)), "");
        assert_eq!(idx_padding(Some(1)), " ");
        assert_eq!(idx_padding(Some(2)), "  ");
        assert_eq!(idx_padding(Some(3)), "   ");
    }

    #[test]
    fn test_signs() {
        println!("{FILE_SIGN}\n{SIDE_PADDING_SIGN}\n{SIDE_SIGN}\n{SIDE_PADDING_SIGN}\n{HELP_SIGN}");
    }

    use crate::parse::Wrong;
    use crate::{pfe, ple, pwe};
    #[test]
    fn test_new() {
        // dbg!(pfe!("test"));
        // dbg!(pfe!("test", h:"help", f:"file"));
        // dbg!(pfe!("test", h:"help", f:"file", l:ple!("line")));
        // dbg!(pfe!("test", h:"help", f:"file", l:ple!("line", i:1, w:Wrong::Bit(vec![(42, 3)]))));
        dbg!(pfe!("test", h:"help", f:"file", l:ple!("line", i:1, w:pwe!((42, 3)))));
        dbg!(pfe!("test", h:"help", f:"file", l:ple!("line", i:1, w:pwe!((42, 3), (43, 4)))));
        dbg!(pfe!("test", h:"help", f:"file", l:ple!("line", i:1, w:pwe!("test"))));
    }
}
