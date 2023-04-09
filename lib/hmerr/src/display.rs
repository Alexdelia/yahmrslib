pub const ERROR: &str = "\x1b[0m\x1b[1;31merror\x1b[39m:\x1b[0m\t";
pub const WARNING: &str = "\x1b[0m\x1b[1;33mwarning\x1b[39m:\x1b[0m\t";
pub const HELP: &str = "\x1b[0m\x1b[36mhelp\x1b[0m\x1b[1m:\x1b[0m ";
pub const SIDE: &str = "\x1b[0m\x1b[1;34m";
pub const SOURCE: &str = "\x1b[0m\x1b[1;31m╰╴caused by╶╮\x1b[0m\n";

pub const FILE_COLOR: &str = "\x1b[0m\x1b[38;2;161;211;255m";

pub const FILE_SIGN: &str = "\x1b[0m\x1b[1;34m╭╴◊\x1b[0m ";
pub const SIDE_PADDING_SIGN: &str = "\x1b[0m\x1b[1;34m┆\x1b[0m ";
pub const SIDE_SIGN: &str = "\x1b[0m\x1b[1;34m│\x1b[0m ";
pub const HELP_SIGN: &str = "\x1b[0m\x1b[1;34m╧\x1b[0m ";
pub const SOURCE_SIDE_SIGN: &str = "\x1b[0m\x1b[1;31m│\x1b[0m ";

pub const LINT_COLOR: &str = "\x1b[0m\x1b[1;38;2;255;0;64m";
pub const LINT_SIGN: char = '^';

pub const FILE_SOURCE: &str = "\x1b[0m\x1b[2;3mgenerated in: \x1b[1m";

#[macro_export]
macro_rules! err {
	($($arg:tt)*) => {
		eprintln!("{}{}", $crate::display::ERROR, format!($($arg)*));
	};
}

#[macro_export]
macro_rules! errnl {
	($($arg:tt)*) => {
		eprint!("{}{}", $crate::display::ERROR, format!($($arg)*));
	};
}

#[macro_export]
macro_rules! warn {
	($($arg:tt)*) => {
		eprintln!("{}{}", $crate::display::WARNING, format!($($arg)*));
	};
}

#[macro_export]
macro_rules! warnnl {
	($($arg:tt)*) => {
		eprint!("{}{}", $crate::display::WARNING, format!($($arg)*));
	};
}
