mod key_data;
mod new;
pub use key_data::KeyData;

use crate::Keyword;

use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::path::PathBuf;
use std::str::FromStr;

pub trait FileDataKey: Into<usize> + FromStr + Clone + Copy {}

pub struct FileData<K: FileDataKey>(Vec<KeyData>, PhantomData<K>);

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

pub struct SpofedFile<K: FileDataKey> {
    pub path: PathBuf,
    data: FileData<K>,
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

impl<K: FileDataKey> SpofedFile<K> {
    pub fn name(&self) -> String {
        self.path.to_string_lossy().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{ExpectedLine, ExpectedSize, Format, FoundLine, Keyword, Occurence};

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

    impl FileDataKey for RuleTest {}

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

    impl FileDataKey for RuleTest2 {}

    #[test]
    fn test() {
        let mut f = SpofedFile::<RuleTest> {
            path: PathBuf::from("test"),
            data: FileData::new(vec![
                KeyData::new(
                    FoundLine::new(),
                    ExpectedLine::new(
                        Keyword::new("keyword", "desc"),
                        Format::new("format", ExpectedSize::Fixed),
                        Occurence::OneOrMore,
                    ),
                ),
                KeyData::new(
                    FoundLine::new(),
                    ExpectedLine::new(
                        Keyword::new("other", "desc"),
                        Format::new("yes", ExpectedSize::Fixed),
                        Occurence::OneOrMore,
                    ),
                ),
            ]),
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

        // assert_eq!(f[RuleTest2::Zero].data.0.len(), 0);	// compile error
        // assert_eq!(f[RuleTest2::One].data.0.len(), 0);	// compile error
    }
}
