use crate::{NumArr, Sign, Signed, Unsigned};

use std::marker::PhantomData;

// implement from every primitive signed num type to NumArr<Signed>
macro_rules! impl_from_signed {
	($($t:ty)*) => ($(
		impl From<$t> for NumArr<Signed> {
			#[inline]
			fn from(n: $t) -> Self {
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
					sign,
				}
			}
		}
	)*)
}
