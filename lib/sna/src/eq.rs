use crate::{NumArr, Sign, Signed, Unsigned};

impl Eq for NumArr<Signed> {}

impl PartialEq for NumArr<Signed> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (self.sign == other.sign && self.arr == other.arr) || (self.is_zero() && other.is_zero())
    }
}

impl Eq for NumArr<Unsigned> {}

impl PartialEq for NumArr<Unsigned> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.arr == other.arr
    }
}

impl PartialEq<NumArr<Signed>> for NumArr<Unsigned> {
    #[inline]
    fn eq(&self, other: &NumArr<Signed>) -> bool {
        (other.sign == Signed(Sign::Pos) && self.arr == other.arr)
            || (self.is_zero() && other.is_zero())
    }
}

impl PartialEq<NumArr<Unsigned>> for NumArr<Signed> {
    #[inline]
    fn eq(&self, other: &NumArr<Unsigned>) -> bool {
        (self.sign == Signed(Sign::Pos) && self.arr == other.arr)
            || (self.is_zero() && other.is_zero())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Sign;

    #[test]
    fn test_eq_signed() {
        let ina0 = NumArr::<Signed>::from(0);
        let ina1 = NumArr::<Signed>::from(42);
        let ina2 = NumArr::<Signed>::from(42);
        let ina3 = NumArr::<Signed>::from(-42);
        let ina4 = NumArr::<Signed>::from(84);

        assert_eq!(ina1, ina2);
        assert_ne!(ina1, ina3);
        assert_ne!(ina1, ina4);
        assert_ne!(ina3, ina4);
        assert_eq!(ina0.arr, []);
        assert_eq!(ina0.sign, Signed(Sign::Pos));
    }

    #[test]
    fn test_eq_unsigned() {
        let una0 = NumArr::<Unsigned>::from(0u8);
        let una1 = NumArr::<Unsigned>::from(42u8);
        let una2 = NumArr::<Unsigned>::from(42u8);
        let una3 = NumArr::<Unsigned>::from(84u8);

        assert_eq!(una1, una2);
        assert_ne!(una1, una3);
        assert_eq!(una0.arr, []);
    }
}
