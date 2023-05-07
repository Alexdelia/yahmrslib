use crate::sign::{Signed, Unsigned};

pub type Digit = u8;
pub type DigitArr = Vec<Digit>;

#[derive(Debug, Default, Clone)]
pub struct NumArr<S> {
    pub arr: DigitArr,
    pub sign: S,
}

impl NumArr<Unsigned> {
    /// create a new NumArr<Unsigned>
    #[inline]
    pub fn new() -> Self {
        NumArr::default()
    }
}

impl NumArr<Signed> {
    /// create a new NumArr<Signed>
    #[inline]
    pub fn new() -> Self {
        NumArr::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let una = NumArr::<Unsigned>::new();
        let ina = NumArr::<Signed>::new();

        assert_eq!(una, NumArr::<Unsigned>::default());
        assert_eq!(ina, NumArr::<Signed>::default());
    }
}
