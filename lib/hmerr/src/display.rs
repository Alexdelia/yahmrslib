pub const ERROR: &str = "\x1b[0m\x1b[1;31merror\x1b[39m:\x1b[0m\t";
pub const HELP: &str = "\x1b[0m\x1b[36mhelp\x1b[0m:\t";
pub const SIDE: &str = "\x1b[0m\x1b[34m";
pub const SOURCE: &str = "\x1b[0m\x1b[1;31m╰╴caused by\x1b[39m:\x1b[0m\n";

pub const FILE_SIGN: &str = "\x1b[0m\x1b[1;34m╭╴◊\x1b[0m ";
pub const SIDE_PADDING_SIGN: &str = "\x1b[0m\x1b[1;34m┆\x1b[0m ";
pub const SIDE_SIGN: &str = "\x1b[0m\x1b[1;34m│\x1b[0m ";
pub const HELP_SIGN: &str = "\x1b[0m\x1b[1;34m╧\x1b[0m ";

pub const LINT_COLOR: &str = "\x1b[0m\x1b[1;33m";
pub const LINT_SIGN: char = '⚠';

pub const FILE_SOURCE: &str = "\x1b[0m\x1b[2;3mcomming from:\t\x1b[1m";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signs() {
        println!("{FILE_SIGN}\n{SIDE_PADDING_SIGN}\n{SIDE_SIGN}\n{SIDE_PADDING_SIGN}\n{HELP_SIGN}");
    }

    use crate::{pfe, ple, pwe};
    #[test]
    fn test_new() {
        dbg!(pfe!("test"));
        dbg!(pfe!("test", h:"help", f:"file"));
        dbg!(pfe!("test", h:"help", f:"file", l:ple!("line")));
        dbg!(pfe!("test", h:"help", f:"file", l:ple!("line", i:1, w:pwe!((42, 3)))));
        dbg!(pfe!("test", h:"help", f:"file", l:ple!("line", i:1, w:pwe!((42, 3), (43, 4)))));
        dbg!(pfe!("test", h:"help", f:"file", l:ple!("line", i:1, w:pwe!("test"))));
        dbg!(
            pfe!("test", h:"help", f:"file", l:ple!("line", i:1, w:pwe!("test", "some other test")))
        );
        dbg!(pfe!("test", h:"help", f:"file", l:ple!("line", i:1, w:pwe!("test", (42, 3)))));
    }
}
