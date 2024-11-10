use std::fmt::{Debug, Display};

use ansi::abbrev::{B, D, G, I, R};

#[derive(Default, PartialEq, Clone, Copy)]
pub enum Occurrence {
	#[default]
	Once, // 1
	Optional,            // 0 or 1
	ZeroOrMore,          // 0 or more
	OneOrMore,           // 1 or more
	Exactly(usize),      // exactly n
	Range(usize, usize), // between start and end
}

impl Display for Occurrence {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Occurrence::Once => write!(f, "once"),
			Occurrence::Optional => write!(f, "optional"),
			Occurrence::ZeroOrMore => write!(f, "zero or more"),
			Occurrence::OneOrMore => write!(f, "one or more"),
			Occurrence::Exactly(x) => write!(f, "exactly {x}"),
			Occurrence::Range(start, end) => write!(f, "between {start} and {end}"),
		}
	}
}

impl Debug for Occurrence {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Occurrence::Once => write!(f, "n == 1"),
			Occurrence::Optional => write!(f, "n <= 1"),
			Occurrence::ZeroOrMore => write!(f, "n >= 0"),
			Occurrence::OneOrMore => write!(f, "n >= 1"),
			Occurrence::Exactly(x) => write!(f, "n == {x}"),
			Occurrence::Range(start, end) => write!(f, "n >= {start} && n <= {end}"),
		}
	}
}

impl Occurrence {
	pub fn in_range(&self, n: usize) -> bool {
		match self {
			Occurrence::Once => n == 1,
			Occurrence::Optional => n <= 1,
			Occurrence::ZeroOrMore => true, // n >= 0
			Occurrence::OneOrMore => n >= 1,
			Occurrence::Exactly(x) => n == *x,
			Occurrence::Range(start, end) => n >= *start && n <= *end,
		}
	}

	pub fn check(&self, n: usize) -> Result<(), String> {
		if self.in_range(n) {
			Ok(())
		} else {
			Err(format!(
				"expected to be {B}{G}{self} {I}({self:?}){D}, but it occurred {B}{R}{n}{D} times",
			))
		}
	}
}

/// macro to create a new Occurrence
///
/// # Example
///
/// ```
/// use spof::{Occurrence, occurrence};
///
/// let occ = occurrence!(Once);
/// assert_eq!(occ, Occurrence::Once);
///
/// let occ = occurrence!(Optional);
/// assert_eq!(occ, Occurrence::Optional);
///
/// let occ = occurrence!(ZeroOrMore);
/// assert_eq!(occ, Occurrence::ZeroOrMore);
///
/// let occ = occurrence!(OneOrMore);
/// assert_eq!(occ, Occurrence::OneOrMore);
///
/// let occ = occurrence!(n:42);
/// assert_eq!(occ, Occurrence::Exactly(42));
///
/// let occ = occurrence!(42);
/// assert_eq!(occ, Occurrence::Exactly(42));
///
/// let occ = occurrence!((42));
/// assert_eq!(occ, Occurrence::Exactly(42));
///
/// let occ = occurrence!(1, 42);
/// assert_eq!(occ, Occurrence::Range(1, 42));
///
/// let n = 42;
/// let occ = occurrence!(n:n);
/// assert_eq!(occ, Occurrence::Exactly(n));
///
/// let min = 1;
/// let max = 42;
/// let occ = occurrence!(min, max);
/// assert_eq!(occ, Occurrence::Range(min, max));
///
/// for occ in vec![
///    Occurrence::Once,
///    Occurrence::Optional,
///    Occurrence::ZeroOrMore,
///    Occurrence::OneOrMore,
///    Occurrence::Exactly(42),
///    Occurrence::Range(1, 42),
/// ] {
///    assert_eq!(occ, occurrence!(occ));
/// }
/// ```
#[macro_export]
macro_rules! occurrence {
	(Once) => {
		$crate::Occurrence::Once
	};
	(Optional) => {
		$crate::Occurrence::Optional
	};
	(ZeroOrMore) => {
		$crate::Occurrence::ZeroOrMore
	};
	(OneOrMore) => {
		$crate::Occurrence::OneOrMore
	};
	(n:$n:expr) => {
		$crate::Occurrence::Exactly($n)
	};
	( ($n:expr) ) => {
		$crate::Occurrence::Exactly($n)
	};
	($n:literal) => {
		$crate::Occurrence::Exactly($n)
	};
	($min:expr, $max:expr) => {
		$crate::Occurrence::Range($min, $max)
	};
	( ($min:expr, $max:expr) ) => {
		$crate::Occurrence::Range($min, $max)
	};
	($occ:expr) => {
		$occ
	};
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_occurrence_once() {
		let occ = Occurrence::Once;
		assert!(!occ.in_range(0));
		assert!(occ.in_range(1));
		assert!(!occ.in_range(2));
		assert!(!occ.in_range(42));
	}

	#[test]
	fn test_occurrence_optional() {
		let occ = Occurrence::Optional;
		assert!(occ.in_range(0));
		assert!(occ.in_range(1));
		assert!(!occ.in_range(2));
		assert!(!occ.in_range(42));
	}

	#[test]
	fn test_occurrence_zero_or_more() {
		let occ = Occurrence::ZeroOrMore;
		assert!(occ.in_range(0));
		assert!(occ.in_range(1));
		assert!(occ.in_range(2));
		assert!(occ.in_range(42));
	}

	#[test]
	fn test_occurrence_one_or_more() {
		let occ = Occurrence::OneOrMore;
		assert!(!occ.in_range(0));
		assert!(occ.in_range(1));
		assert!(occ.in_range(2));
		assert!(occ.in_range(42));
	}

	#[test]
	fn test_occurrence_exactly() {
		let occ = Occurrence::Exactly(42);
		assert!(!occ.in_range(0));
		assert!(!occ.in_range(1));
		assert!(!occ.in_range(2));
		assert!(occ.in_range(42));
		assert!(!occ.in_range(84));
	}

	#[test]
	fn test_occurrence_range() {
		let occ = Occurrence::Range(1, 42);
		assert!(!occ.in_range(0));
		assert!(occ.in_range(1));
		assert!(occ.in_range(2));
		assert!(occ.in_range(42));
		assert!(!occ.in_range(84));
	}
}
