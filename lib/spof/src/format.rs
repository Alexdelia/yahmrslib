use std::fmt::{Display, Formatter};

pub struct Format {
    pub token: String,
    size: Size,
}

pub enum Size {
    Fixed(usize),        // expected_size == size
    Undefined,           // expected_size >= 0
    Range(usize, usize), // expected_size >= min && expected_size <= max
}

#[derive(Default)]
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

    pub fn check(&self, line: &str) -> std::result::Result<Vec<String>, (String, usize)> {
        let split: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

        self.size.check(split.len())?;
        Ok(split)
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

    impl std::fmt::Debug for Size {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Fixed(expected_size) => write!(f, "Fixed({})", expected_size),
                Self::Undefined => write!(f, "Undefined"),
                Self::Range(min, max) => write!(f, "Range({}, {})", min, max),
            }
        }
    }

    #[test]
    fn test_format_fixed() {
        let format = Format::new("test", ExpectedSize::Fixed);
        assert_eq!(format.token, "test");
        assert_eq!(format.size, Size::Fixed(1));
        assert!(format.check("test").is_ok());
        assert!(format.check("test test").is_err());

        let format = Format::new("test test", ExpectedSize::Fixed);
        assert_eq!(format.token, "test test");
        assert_eq!(format.size, Size::Fixed(2));
        assert!(format.check("test test").is_ok());
        assert!(format.check("test").is_err());
    }

    #[test]
    fn test_format_undefined() {
        let format = Format::new("test", ExpectedSize::Undefined);
        assert_eq!(format.token, "test");
        assert_eq!(format.size, Size::Undefined);
        assert!(format.check("test").is_ok());
        assert!(format.check("test test").is_ok());
    }

    #[test]
    fn test_format_range() {
        let format = Format::new("test", ExpectedSize::Range(1, 3));
        assert_eq!(format.token, "test");
        assert_eq!(format.size, Size::Range(1, 3));
        assert!(format.check("test").is_ok());
        assert!(format.check("test test").is_ok());
        assert!(format.check("test test test").is_ok());
        assert!(format.check("test test test test").is_err());
        assert!(format.check("").is_err());
    }
}
