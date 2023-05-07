use crate::num_arr::Digit;
use crate::NumArr;

use std::ops::{Index, IndexMut};

impl<S> Index<usize> for NumArr<S> {
    type Output = Digit;

    /// indexing for NumArr<S>
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.arr[index]
    }
}

impl<S> IndexMut<usize> for NumArr<S> {
    /// mutable indexing for NumArr<S>
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.arr[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Signed;

    #[test]
    fn test_index() {
        let mut num_arr = NumArr::<Signed>::from(123);
        assert_eq!(num_arr[0], 3);
        assert_eq!(num_arr[1], 2);
        assert_eq!(num_arr[2], 1);

        num_arr[0] = 4;
        assert_eq!(num_arr[0], 4);
    }
}
