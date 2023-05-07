use crate::{NumArr, Sign, Signed};

use std::ops::Neg;

impl Neg for Sign {
    type Output = Self;

    /// Negate a Sign
    #[inline]
    fn neg(self) -> Self::Output {
        match self {
            Sign::Pos => Sign::Neg,
            Sign::Neg => Sign::Pos,
        }
    }
}

impl Neg for Signed {
    type Output = Self;

    /// Negate a Signed
    #[inline]
    fn neg(self) -> Self::Output {
        match self.0 {
            Sign::Pos => Signed(Sign::Neg),
            Sign::Neg => Signed(Sign::Pos),
        }
    }
}

impl Neg for NumArr<Signed> {
    type Output = Self;

    /// Negate a NumArr<Signed>
    #[inline]
    fn neg(self) -> Self::Output {
        NumArr {
            arr: self.arr,
            sign: -self.sign,
        }
    }
}

impl Neg for &NumArr<Signed> {
    type Output = NumArr<Signed>;

    /// Negate a &NumArr<Signed>
    #[inline]
    fn neg(self) -> Self::Output {
        -self.clone()
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neg() {
        let ina = NumArr::<Signed>::from(42);
        assert_eq!(-ina, NumArr::<Signed>::from(-42));

        let ina = NumArr::<Signed>::from(-42);
        assert_eq!(-ina, NumArr::<Signed>::from(42));

        let ina = NumArr::<Signed>::from(0);
        assert_eq!(-ina, NumArr::<Signed>::from(0));
    }
}
*/
