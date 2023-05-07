use crate::{NumArr, Sign, Signed, Unsigned};

macro_rules! impl_from_signed {
	($($t:ty)*) => ($(
		impl From<$t> for NumArr<Signed> {
			#[inline]
			fn from(n: $t) -> Self {
				if n == 0 {
					return NumArr {
						arr: vec![0],
						sign: Signed(Sign::Pos),
					};
				}

				let mut arr = Vec::new();
				let mut n = n;
				let mut sign = Sign::Pos;

				if n < 0 {
					sign = Sign::Neg;
					n = -n;
				}

				while n > 0 {
					arr.push((n % 10) as u8);
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

impl_from_signed!(i8 i16 i32 i64 i128 isize);

macro_rules! impl_from_unsigned {
	($($t:ty)*) => ($(
		impl From<$t> for NumArr<Unsigned> {
			#[inline]
			fn from(n: $t) -> Self {
				if n == 0 {
					return NumArr {
						arr: vec![0],
						sign: Unsigned,
					};
				}

				let mut arr = Vec::new();
				let mut n = n;

				while n > 0 {
					arr.push((n % 10) as u8);
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

impl_from_unsigned!(u8 u16 u32 u64 u128 usize);
