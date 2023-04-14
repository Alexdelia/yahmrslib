pub mod expected_line;
pub mod format;
pub mod keyword;
pub mod occurence;

use crate::{ExpectedLine, Keyword};

use std::collections::HashMap;

pub struct Rule(HashMap<String, ExpectedLine>);

impl Rule {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add(&mut self, el: ExpectedLine) {
        self.0.insert(el.k.keyword.clone(), el);
    }

    pub fn get(&self, keyword: &str) -> Option<&ExpectedLine> {
        self.0.get(keyword)
    }

    pub fn keywords(&self) -> Vec<&Keyword> {
        self.0.values().map(|el| &el.k).collect()
    }
}

/// create file rule
///
/// # Arguments
///
/// * `k` - keyword
/// * `f` - expected format
/// * `s` - expected format token size
/// * `o` - expected line occurence
/// * `d` - keyword description
///
/// # Example
///
/// ```
/// use crate::{rule, Rule, FileData};
///
/// let r: Rule = rule!(
///    ("color", "R G B", Fixed, Once, "the color of the object"),
///    ("position", "X Y Z [W]", (3, 4), Once, "the position of the object"),	// don't need '[', ']' in format to be optional
///    ("name", "string", OneOrMore, Optional, "the name of the object")
/// );
///
/// /* valid file:
///     color 255 0 42
///     # some comment
///     position 1 2 3 4 # another comment
///     name my object
/// */
///
/// /* invalid file:
///    color 255 0 42 255  # too many values
///    position 1 2        # not enough values
///    name                # missing value
///    color 255 0 42      # defined twice (Once is defined in the rule)
/// */
/// ```
#[macro_export]
macro_rules! rule {
	($($k:expr, $f:expr, $s:expr, $o:expr, $d:expr),*) => {
		{
			let mut r = $crate::Rule::new();
			$(
				r.add($crate::ExpectedLine::new(
					$crate::Keyword::new($k, $d),
					$crate::Format::new($f, $crate::expected_size!( $s )),
					$crate::occurence!( $o )
				));
			)*
			r
		}
	};
}
