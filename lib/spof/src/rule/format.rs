use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct Format {
    pub token: String,
    size: Size,
}

pub enum Size {
    Fixed(usize),        // expected_size == size
    Undefined,           // expected_size >= 0
    Range(usize, usize), // expected_size >= min && expected_size <= max
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum ExpectedSize {
    #[default]
    Fixed,
    Undefined,
    Range(usize, usize),
}

impl From<(ExpectedSize, &str)> for Size {
    fn from((in_size, token): (ExpectedSize, &str)) -> Self {
        match in_size {
            ExpectedSize::Fixed => {
                Self::Fixed(token.split_whitespace().collect::<Vec<&str>>().len())
            }
            ExpectedSize::Undefined => Self::Undefined,
            ExpectedSize::Range(min, max) => Self::Range(min, max),
        }
    }
}

impl Format {
    pub fn new(token: impl Into<String>, expected_size: ExpectedSize) -> Self {
        let token: String = token.into();
        let size: Size = (expected_size, token.as_str()).into();
        Self { token, size }
    }

    pub fn check(&self, token: &Vec<String>) -> std::result::Result<(), (String, usize)> {
        self.size.check(token.len())
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fixed(expected_size) => write!(f, "{}", expected_size),
            Self::Undefined => write!(f, "undefined"),
            Self::Range(min, max) => write!(f, "{}-{}", min, max),
        }
    }
}

impl Debug for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fixed(expected_size) => write!(f, "Fixed({})", expected_size),
            Self::Undefined => write!(f, "Undefined"),
            Self::Range(min, max) => write!(f, "Range({}, {})", min, max),
        }
    }
}

impl Size {
    fn in_range(&self, size: usize) -> bool {
        match self {
            Self::Fixed(expected_size) => size == *expected_size,
            Self::Undefined => true,
            Self::Range(min, max) => size >= *min && size <= *max,
        }
    }

    fn check(&self, size: usize) -> std::result::Result<(), (String, usize)> {
        if self.in_range(size) {
            Ok(())
        } else {
            Err((format!("{self}"), size))
        }
    }
}

/// macro to create a new ExpectedSize
///
/// # Example
///
/// ```
/// use spof::{ExpectedSize, expected_size};
///
/// let es = expected_size!(Fixed);
/// assert_eq!(es, ExpectedSize::Fixed);
///
/// let es = expected_size!(Undefined);
/// assert_eq!(es, ExpectedSize::Undefined);
///
/// let es = expected_size!(1, 3);
/// assert_eq!(es, ExpectedSize::Range(1, 3));
///
/// let es = expected_size!((1, 3));
/// assert_eq!(es, ExpectedSize::Range(1, 3));
///
/// let var = ExpectedSize::Fixed;
/// let es = expected_size!(var);
/// assert_eq!(es, ExpectedSize::Fixed);
///
/// let var = ExpectedSize::Undefined;
/// let es = expected_size!(var);
/// assert_eq!(es, ExpectedSize::Undefined);
///
/// let var = ExpectedSize::Range(1, 3);
/// let es = expected_size!(var);
/// assert_eq!(es, ExpectedSize::Range(1, 3));
///
/// let min = 1;
/// let max = 3;
/// let es = expected_size!(min, max);
/// assert_eq!(es, ExpectedSize::Range(1, 3));
///
/// let es = expected_size!(ExpectedSize::Fixed);
/// assert_eq!(es, ExpectedSize::Fixed);
///
/// let es = expected_size!(ExpectedSize::Undefined);
/// assert_eq!(es, ExpectedSize::Undefined);
///
/// let es = expected_size!(ExpectedSize::Range(1, 3));
/// assert_eq!(es, ExpectedSize::Range(1, 3));
/// ```
#[macro_export]
macro_rules! expected_size {
    (Fixed) => {
        $crate::ExpectedSize::Fixed
    };
    (Undefined) => {
        $crate::ExpectedSize::Undefined
    };
    ($min:expr, $max:expr) => {
        $crate::ExpectedSize::Range($min, $max)
    };
    ( ($min:expr, $max:expr) ) => {
        $crate::ExpectedSize::Range($min, $max)
    };
    ($var:expr) => {
        $var
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_size_fixed() {
        let size = Size::Fixed(1);
        assert!(!size.in_range(0));
        assert!(size.in_range(1));
        assert!(!size.in_range(2));
        assert!(!size.in_range(42));
    }

    #[test]
    fn test_size_undefined() {
        let size = Size::Undefined;
        assert!(size.in_range(0));
        assert!(size.in_range(1));
        assert!(size.in_range(2));
        assert!(size.in_range(42));
    }

    #[test]
    fn test_size_range() {
        let size = Size::Range(1, 3);
        assert!(!size.in_range(0));
        assert!(size.in_range(1));
        assert!(size.in_range(2));
        assert!(size.in_range(3));
        assert!(!size.in_range(4));
        assert!(!size.in_range(42));
    }

    impl PartialEq for Size {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Fixed(a), Self::Fixed(b)) => a == b,
                (Self::Undefined, Self::Undefined) => true,
                (Self::Range(a_min, a_max), Self::Range(b_min, b_max)) => {
                    a_min == b_min && a_max == b_max
                }
                _ => false,
            }
        }
    }

    #[test]
    fn test_format_fixed() {
        let token = String::from("test");
        let format = Format::new("test", ExpectedSize::Fixed);
        assert_eq!(format.token, "test");
        assert_eq!(format.size, Size::Fixed(1));
        assert!(format.check(&vec![]).is_err());
        assert!(format.check(&vec![token.clone()]).is_ok());
        assert!(format.check(&vec![token.clone(); 2]).is_err());
        assert!(format.check(&vec![token.clone(); 3]).is_err());

        let format = Format::new("test test", ExpectedSize::Fixed);
        assert_eq!(format.token, "test test");
        assert_eq!(format.size, Size::Fixed(2));
        assert!(format.check(&vec![]).is_err());
        assert!(format.check(&vec![token.clone()]).is_err());
        assert!(format.check(&vec![token.clone(); 2]).is_ok());
        assert!(format.check(&vec![token; 3]).is_err());
    }

    #[test]
    fn test_format_undefined() {
        let token = String::from("test");
        let format = Format::new("test", ExpectedSize::Undefined);
        assert_eq!(format.token, "test");
        assert_eq!(format.size, Size::Undefined);
        assert!(format.check(&vec![]).is_ok());
        assert!(format.check(&vec![token.clone()]).is_ok());
        assert!(format.check(&vec![token.clone(); 2]).is_ok());
        assert!(format.check(&vec![token; 3]).is_ok());
    }

    #[test]
    fn test_format_range() {
        let token = String::from("test");
        let format = Format::new("test", ExpectedSize::Range(1, 3));
        assert_eq!(format.token, "test");
        assert_eq!(format.size, Size::Range(1, 3));
        assert!(format.check(&vec![]).is_err());
        assert!(format.check(&vec![token.clone()]).is_ok());
        assert!(format.check(&vec![token.clone(); 2]).is_ok());
        assert!(format.check(&vec![token.clone(); 3]).is_ok());
        assert!(format.check(&vec![token; 4]).is_err());
    }

    #[test]
    fn test_empty_format() {
        let token = String::from("test");
        let format = Format::new("", ExpectedSize::Fixed);
        assert_eq!(format.token, "");
        assert_eq!(format.size, Size::Fixed(0));
        assert!(format.check(&vec![]).is_ok());
        assert!(format.check(&vec![token.clone()]).is_err());
        assert!(format.check(&vec![token.clone(); 2]).is_err());
        assert!(format.check(&vec![token; 3]).is_err());
    }
}
