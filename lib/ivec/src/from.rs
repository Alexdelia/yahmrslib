use super::IVec;

impl<T> From<Vec<T>> for IVec<T> {
    fn from(vec: Vec<T>) -> Self {
        IVec { vec, i: 0 }
    }
}

impl<T> From<IVec<T>> for Vec<T> {
    fn from(ivec: IVec<T>) -> Self {
        ivec.vec
    }
}
