use crate::sign::{Signed, Unsigned};

use std::marker::PhantomData;

#[derive(Debug, Default, Clone)]
pub struct NumArr<S> {
    pub arr: Vec<u8>,
    pub sign: S,
}

impl Eq for NumArr<Signed> {}

impl PartialEq for NumArr<Signed> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (self.sign == other.sign && self.arr == other.arr)
            || ((self.arr.is_empty() || self.arr == [0])
                && (other.arr.is_empty() || other.arr == [0]))
    }
}

impl PartialEq for NumArr<Unsigned> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.arr == other.arr
    }
}

impl NumArr<Unsigned> {
    /// Create a new NumArr<Unsigned>
    #[inline]
    pub fn new() -> Self {
        NumArr::default()
    }
}

impl NumArr<Signed> {
    /// Create a new NumArr<Signed>
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
