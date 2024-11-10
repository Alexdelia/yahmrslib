use super::{KeyData, SpofedFile};

use crate::Keyword;

use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

pub trait FileDataKey: Into<usize> + FromStr + Clone + Copy {
    fn build() -> FileData<Self>;
}

pub struct FileData<K: FileDataKey>(Vec<KeyData>, PhantomData<K>);

impl<K: FileDataKey> std::fmt::Debug for FileData<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = String::new();

        for k in self.0.iter() {
            s.push_str(&format!("     ├{key}\n", key = k.rule.k.keyword));

            for f in &k.data.0 {
                s.push_str(&format!(
                    "{l:>5}│ {token}\n",
                    l = f.1,
                    token = f.0.join(" ")
                ));
            }

            s.push_str("     ┆\n");
        }

        write!(f, "{s}")
    }
}

impl<K: FileDataKey> FileData<K> {
    pub fn new(data: Vec<KeyData>) -> Self {
        Self(data, PhantomData)
    }

    pub fn keywords(&self) -> Vec<&Keyword> {
        self.0.iter().map(|k| &k.rule.k).collect()
    }
}

impl<K: FileDataKey> Index<K> for FileData<K> {
    type Output = KeyData;

    fn index(&self, index: K) -> &Self::Output {
        &self.0[index.into()]
    }
}

impl<K: FileDataKey> IndexMut<K> for FileData<K> {
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        &mut self.0[index.into()]
    }
}

impl<K: FileDataKey> Index<K> for SpofedFile<K> {
    type Output = KeyData;

    fn index(&self, index: K) -> &Self::Output {
        &self.data.0[index.into()]
    }
}

impl<K: FileDataKey> IndexMut<K> for SpofedFile<K> {
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        &mut self.data.0[index.into()]
    }
}

/// create SpofedFile rule
///
/// # Arguments
///
/// * `enum_name:ident` - enum name
/// * `keyword:ident` - keyword identifier
/// * `k:literal` - keyword string
/// * `f:expr` - expected format string
/// * `s:tt` - expected format token size
/// * `o:tt` - expected line occurrence
/// * `d:expr` - keyword description
///
/// # Example
///
/// ```
/// use spof::rule;
///
/// rule!(
///     enum RuleTest {
///         Color => "color", "R G B", Fixed, Once, "the color of the object",
///         Position => "position", "X Y Z [W]", (3, 4), Once, "the position of the object", // don't need '[', ']' in format to be optional
///         Name => "name", "string", Undefined, Optional, "the name of the object",
///     }
/// );
///
/// use spof::FileDataKey;    // must bring trait into scope to use build()
/// let r = RuleTest::build();
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
	( $p:vis enum $enum_name:ident { $( $key_enum:ident => $k:expr, $f:expr, $s:tt, $o:tt, $d:expr ),* $(,)? } ) => {
		#[derive(Debug, Clone, Copy, PartialEq, Eq)]
		$p enum $enum_name {
			$( $key_enum ),*
		}

		impl ::std::convert::Into<usize> for $enum_name {
			fn into(self) -> usize {
				self as usize
			}
		}

		impl ::std::str::FromStr for $enum_name {
			type Err = ();

			fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
				match s {
					$( $k => Ok($enum_name::$key_enum), )*
					_ => Err(()),
				}
			}
		}

		impl $crate::FileDataKey for $enum_name {
			fn build() -> $crate::FileData<Self> {
				$crate::FileData::new(vec![
					$(
						$crate::KeyData::new(
							$crate::FoundLine::new(),
							$crate::ExpectedLine::new(
								$crate::Keyword::new($k, $d),
								$crate::Format::new($f, $crate::expected_size!($s)),
								$crate::occurrence!($o),
							),
						),
					)*
				])
			}
		}
	};
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{ExpectedLine, ExpectedSize, Format, FoundLine, Keyword, Occurrence};
    use std::path::PathBuf;

    #[derive(Clone, Copy)]
    enum RuleTest {
        Zero,
        One,
    }

    impl Into<usize> for RuleTest {
        fn into(self) -> usize {
            self as usize
        }
    }

    impl FromStr for RuleTest {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "Zero" => Ok(RuleTest::Zero),
                "One" => Ok(RuleTest::One),
                _ => Err(()),
            }
        }
    }

    impl FileDataKey for RuleTest {
        fn build() -> FileData<Self> {
            FileData::new(vec![
                KeyData::new(
                    FoundLine::new(),
                    ExpectedLine::new(
                        Keyword::new("Zero", "the zero"),
                        Format::new("0", ExpectedSize::Fixed),
                        Occurrence::Once,
                    ),
                ),
                KeyData::new(
                    FoundLine::new(),
                    ExpectedLine::new(
                        Keyword::new("One", "the one"),
                        Format::new("1", ExpectedSize::Fixed),
                        Occurrence::Once,
                    ),
                ),
            ])
        }
    }

    #[allow(dead_code)]
    #[derive(Clone, Copy)]
    enum RuleTest2 {
        Zero,
        One,
    }

    impl Into<usize> for RuleTest2 {
        fn into(self) -> usize {
            self as usize
        }
    }

    impl FromStr for RuleTest2 {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "Zero" => Ok(RuleTest2::Zero),
                "One" => Ok(RuleTest2::One),
                _ => Err(()),
            }
        }
    }

    impl FileDataKey for RuleTest2 {
        fn build() -> FileData<Self> {
            FileData::new(vec![
                KeyData::new(
                    FoundLine::new(),
                    ExpectedLine::new(
                        Keyword::new("Zero", "the zero"),
                        Format::new("0", ExpectedSize::Fixed),
                        Occurrence::Once,
                    ),
                ),
                KeyData::new(
                    FoundLine::new(),
                    ExpectedLine::new(
                        Keyword::new("One", "the one"),
                        Format::new("1", ExpectedSize::Fixed),
                        Occurrence::Once,
                    ),
                ),
            ])
        }
    }

    #[test]
    fn test() {
        let mut f = SpofedFile::<RuleTest> {
            path: PathBuf::from("test"),
            data: RuleTest::build(),
        };

        f[RuleTest::Zero].data = FoundLine::new();
        f[RuleTest::Zero]
            .data
            .push((vec![String::from("keyword"), String::from("format")], 0));
        f[RuleTest::One].data = FoundLine::new();
        f[RuleTest::One]
            .data
            .push((vec![String::from("other"), String::from("yes")], 0));
        assert_eq!(f[RuleTest::Zero].data.0[0].0[0], "keyword");
        assert_eq!(f[RuleTest::One].data.0[0].0[0], "other");

        // assert_eq!(f[RuleTest2::Zero].data.0.len(), 0);  // compile error
        // assert_eq!(f[RuleTest2::One].data.0.len(), 0);	// compile error
    }
}
