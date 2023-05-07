use crate::{NumArr, Sign, Signed, Unsigned};

use num_traits::{One, Zero};

impl Zero for NumArr<Unsigned> {
    #[inline]
    fn zero() -> NumArr<Unsigned> {
        NumArr {
            arr: Vec::new(),
            sign: Unsigned,
        }
    }

    #[inline]
    fn set_zero(&mut self) {
        self.arr.clear();
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.arr.is_empty()
    }
}

impl Zero for NumArr<Signed> {
    #[inline]
    fn zero() -> NumArr<Signed> {
        NumArr {
            arr: Vec::new(),
            sign: Signed(Sign::Pos),
        }
    }

    #[inline]
    fn set_zero(&mut self) {
        self.arr.clear();
        self.sign = Signed(Sign::Pos);
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.arr.is_empty()
    }
}

impl One for NumArr<Unsigned> {
    #[inline]
    fn one() -> NumArr<Unsigned> {
        NumArr {
            arr: vec![1],
            sign: Unsigned,
        }
    }

    #[inline]
    fn set_one(&mut self) {
        self.arr.clear();
        self.arr.push(1);
    }

    #[inline]
    fn is_one(&self) -> bool {
        self.arr[..] == [1]
    }
}

impl One for NumArr<Signed> {
    #[inline]
    fn one() -> NumArr<Signed> {
        NumArr {
            arr: vec![1],
            sign: Signed(Sign::Pos),
        }
    }

    #[inline]
    fn set_one(&mut self) {
        self.arr.clear();
        self.arr.push(1);
        self.sign = Signed(Sign::Pos);
    }

    #[inline]
    fn is_one(&self) -> bool {
        self.arr[..] == [1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        let una = NumArr::<Unsigned>::zero();
        let ina = NumArr::<Signed>::zero();

        assert_eq!(una, NumArr::<Unsigned>::default());
        assert_eq!(ina, NumArr::<Signed>::default());
    }

    #[test]
    fn test_set_zero() {
        let mut una = NumArr::<Unsigned>::from(42);
        let mut ina = NumArr::<Signed>::from(42);

        una.set_zero();
        ina.set_zero();

        assert_eq!(una, NumArr::<Unsigned>::default());
        assert_eq!(ina, NumArr::<Signed>::default());
    }

    #[test]
    fn test_is_zero() {
        let una0 = NumArr::<Unsigned>::from(0u8);
        let una1 = NumArr::<Unsigned>::from(42u8);
        let ina0 = NumArr::<Signed>::from(0);
        let ina1 = NumArr::<Signed>::from(42);

        assert!(una0.is_zero());
        assert!(!una1.is_zero());
        assert!(ina0.is_zero());
        assert!(!ina1.is_zero());
    }

    #[test]
    fn test_one() {
        let una = NumArr::<Unsigned>::one();
        let ina = NumArr::<Signed>::one();

        assert_eq!(una.arr, [1]);
        assert_eq!(una.sign, Unsigned);
        assert_eq!(ina.arr, [1]);
        assert_eq!(ina.sign, Signed(Sign::Pos));
    }

    #[test]
    fn test_set_one() {
        let mut una = NumArr::<Unsigned>::from(42);
        let mut ina = NumArr::<Signed>::from(42);

        una.set_one();
        ina.set_one();

        assert_eq!(una.arr, [1]);
        assert_eq!(una.sign, Unsigned);
        assert_eq!(ina.arr, [1]);
        assert_eq!(ina.sign, Signed(Sign::Pos));
    }

    #[test]
    fn test_is_one() {
        let una0 = NumArr::<Unsigned>::from(0u8);
        let una1 = NumArr::<Unsigned>::from(1u8);
        let una2 = NumArr::<Unsigned>::from(42u8);
        let ina0 = NumArr::<Signed>::from(0);
        let ina1 = NumArr::<Signed>::from(1);
        let ina2 = NumArr::<Signed>::from(42);

        assert!(!una0.is_one());
        assert!(una1.is_one());
        assert!(!una2.is_one());
        assert!(!ina0.is_one());
        assert!(ina1.is_one());
        assert!(!ina2.is_one());
    }
}
