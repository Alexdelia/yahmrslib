use crate::num_arr::Digit;
use crate::{NumArr, Sign, Signed, Unsigned};

macro_rules! impl_from_i_to_signed {
	($($t:ty)*) => ($(
		impl From<$t> for NumArr<Signed> {
			fn from(n: $t) -> Self {
				let mut arr = Vec::new();
				let mut n = n;
				let mut sign = Sign::Pos;

				if n < 0 {
					sign = Sign::Neg;
					n = -n;
				}

				while n > 0 {
					arr.push((n % 10) as Digit);
					n /= 10;
				}

				NumArr {
					arr,
					sign: Signed(sign),
				}
			}
		}
	)*)
}

macro_rules! impl_from_u_to_signed {
	($($t:ty)*) => ($(
		impl From<$t> for NumArr<Signed> {
			fn from(n: $t) -> Self {
				let mut arr = Vec::new();
				let mut n = n;

				while n > 0 {
					arr.push((n % 10) as Digit);
					n /= 10;
				}

				NumArr {
					arr,
					sign: Signed(Sign::Pos),
				}
			}
		}
	)*)
}

macro_rules! impl_from_u_to_unsigned {
	($($t:ty)*) => ($(
		impl From<$t> for NumArr<Unsigned> {
			fn from(n: $t) -> Self {
				let mut arr = Vec::new();
				let mut n = n;

				while n > 0 {
					arr.push((n % 10) as Digit);
					n /= 10;
				}

				NumArr {
					arr,
					sign: Unsigned,
				}
			}
		}
	)*)
}

impl_from_i_to_signed!(i8 i16 i32 i64 i128 isize);
impl_from_u_to_signed!(u8 u16 u32 u64 u128 usize);
impl_from_u_to_unsigned!(u8 u16 u32 u64 u128 usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_i_to_signed() {
        let ina = NumArr::<Signed>::from(42i8);
        assert_eq!(ina.arr, vec![2, 4]);
        assert_eq!(ina.sign, Signed(Sign::Pos));

        let ina = NumArr::<Signed>::from(-42i16);
        assert_eq!(ina.arr, vec![2, 4]);
        assert_eq!(ina.sign, Signed(Sign::Neg));

        let ina = NumArr::<Signed>::from(42i32);
        assert_eq!(ina.arr, vec![2, 4]);
        assert_eq!(ina.sign, Signed(Sign::Pos));

        let ina = NumArr::<Signed>::from(-42i64);
        assert_eq!(ina.arr, vec![2, 4]);
        assert_eq!(ina.sign, Signed(Sign::Neg));

        let ina = NumArr::<Signed>::from(42i128);
        assert_eq!(ina.arr, vec![2, 4]);
        assert_eq!(ina.sign, Signed(Sign::Pos));

        let ina = NumArr::<Signed>::from(-42isize);
        assert_eq!(ina.arr, vec![2, 4]);
        assert_eq!(ina.sign, Signed(Sign::Neg));
    }

    #[test]
    fn test_from_u_to_signed() {
        let ina = NumArr::<Signed>::from(42u8);
        assert_eq!(ina.arr, vec![2, 4]);
        assert_eq!(ina.sign, Signed(Sign::Pos));

        let ina = NumArr::<Signed>::from(42u16);
        assert_eq!(ina.arr, vec![2, 4]);
        assert_eq!(ina.sign, Signed(Sign::Pos));

        let ina = NumArr::<Signed>::from(42u32);
        assert_eq!(ina.arr, vec![2, 4]);
        assert_eq!(ina.sign, Signed(Sign::Pos));

        let ina = NumArr::<Signed>::from(42u64);
        assert_eq!(ina.arr, vec![2, 4]);
        assert_eq!(ina.sign, Signed(Sign::Pos));

        let ina = NumArr::<Signed>::from(42u128);
        assert_eq!(ina.arr, vec![2, 4]);
        assert_eq!(ina.sign, Signed(Sign::Pos));

        let ina = NumArr::<Signed>::from(42usize);
        assert_eq!(ina.arr, vec![2, 4]);
        assert_eq!(ina.sign, Signed(Sign::Pos));
    }

    #[test]
    fn test_from_u_to_unsigned() {
        let ina = NumArr::<Unsigned>::from(42u8);
        assert_eq!(ina.arr, vec![2, 4]);
        assert_eq!(ina.sign, Unsigned);

        let ina = NumArr::<Unsigned>::from(42u16);
        assert_eq!(ina.arr, vec![2, 4]);
        assert_eq!(ina.sign, Unsigned);

        let ina = NumArr::<Unsigned>::from(42u32);
        assert_eq!(ina.arr, vec![2, 4]);
        assert_eq!(ina.sign, Unsigned);

        let ina = NumArr::<Unsigned>::from(42u64);
        assert_eq!(ina.arr, vec![2, 4]);
        assert_eq!(ina.sign, Unsigned);

        let ina = NumArr::<Unsigned>::from(42u128);
        assert_eq!(ina.arr, vec![2, 4]);
        assert_eq!(ina.sign, Unsigned);

        let ina = NumArr::<Unsigned>::from(42usize);
        assert_eq!(ina.arr, vec![2, 4]);
        assert_eq!(ina.sign, Unsigned);
    }
}
