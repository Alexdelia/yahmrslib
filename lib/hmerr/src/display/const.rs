pub const ERROR: &str = "\x1b[0m\x1b[1;31merror\x1b[39m:\x1b[0m\t";
pub const WARNING: &str = "\x1b[0m\x1b[1;33mwarning\x1b[39m:\x1b[0m\t";
pub const HELP: &str = "\x1b[0m\x1b[36mhelp\x1b[0m\x1b[1m:\x1b[0m ";
pub const SIDE: &str = "\x1b[0m\x1b[1;34m";
pub const SOURCE: &str = "\x1b[0m\x1b[1;31m╰╴caused by╶╮\x1b[0m\n";

pub const EXPECTED: &str = "\x1b[0m\t\x1b[2mexpected:\x1b[0m \x1b[1;32m";
pub const GOT: &str = "\x1b[0m\t\x1b[2mgot:\x1b[0m      \x1b[1;31m";
pub const EMPTY_STR: &str = "\x1b[2mnothing\x1b[0m";

pub const FILE_COLOR: &str = "\x1b[0m\x1b[38;2;161;211;255m";

pub const FILE_SIGN: &str = "\x1b[0m\x1b[1;34m╭╴◊\x1b[0m ";
pub const SIDE_PADDING_SIGN: &str = "\x1b[0m\x1b[1;34m┆\x1b[0m ";
pub const SIDE_SIGN: &str = "\x1b[0m\x1b[1;34m│\x1b[0m ";
pub const HELP_SIGN: &str = "\x1b[0m\x1b[1;34m╧\x1b[0m ";
pub const SOURCE_SIDE_SIGN: &str = "\x1b[0m\x1b[1;31m│\x1b[0m ";

pub const LINT_COLOR: &str = "\x1b[0m\x1b[1;38;2;255;0;64m";
pub const LINT_SIGN: char = '^';

pub const FILE_SOURCE: &str = "\x1b[0m\x1b[2;3mgenerated in: \x1b[1m";

pub const TAB: &str = "\x1b[0m\x1b[1;2;35m├──┤\x1b[0m";
pub const SPACE: &str = "\x1b[0m\x1b[1;2;36m·\x1b[0m";
