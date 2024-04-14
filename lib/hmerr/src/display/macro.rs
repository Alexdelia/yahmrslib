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
