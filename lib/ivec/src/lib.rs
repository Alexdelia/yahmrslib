mod from;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IVec<T> {
	pub vec: Vec<T>,
	pub i: usize,
}

impl Default for IVec<usize> {
	fn default() -> Self {
		IVec::new()
	}
}

impl<T> IVec<T> {
	/// construct a new, empty `IVec<T>`
	///
	/// # Example
	/// ```
	/// use ivec::IVec;
	///
	/// let mut ivec = IVec::new();
	///
	/// ivec.vec.push(0);
	/// ivec.vec.push(42);
	/// ivec.vec.push(-21);
	///
	/// assert_eq!(ivec.get(), &0);
	/// assert_eq!(ivec.next(), &42);
	/// assert_eq!(ivec.prev(), &0);
	/// assert_eq!(ivec.to(2), &-21);
	/// ```
	pub fn new() -> Self {
		IVec {
			vec: Vec::new(),
			i: 0,
		}
	}

	#[inline]
	fn get_i(&self) -> usize {
		self.i % self.vec.len()
	}

	#[inline]
	fn set_i(&mut self, i: usize) {
		self.i = i % self.vec.len();
	}

	#[inline]
	fn plus_i(&mut self, i: usize) {
		self.set_i(self.i + i);
	}

	/// get a reference to the element at the current index
	///
	/// # Example
	/// ```
	/// use ivec::IVec;
	///
	/// let mut ivec = IVec::from(vec![0, 42, -21]);
	///
	/// assert_eq!(ivec.get(), &0);
	/// ivec.next();
	/// assert_eq!(ivec.get(), &42);
	/// ivec.next();
	/// assert_eq!(ivec.get(), &-21);
	/// ivec.next();
	/// assert_eq!(ivec.get(), &0);
	/// ```
	#[inline]
	pub fn get(&self) -> &T {
		&self.vec[self.get_i()]
	}

	/// get a mutable reference to the element at the current index
	///
	/// # Example
	/// ```
	/// use ivec::IVec;
	///
	/// let mut ivec = IVec::from(vec![0, 42, -21]);
	///
	/// assert_eq!(ivec.get(), &0);
	/// *ivec.get_mut() = 84;
	/// assert_eq!(ivec.get(), &84);
	/// ```
	#[inline]
	pub fn get_mut(&mut self) -> &mut T {
		let i = self.get_i();
		&mut self.vec[i]
	}

	/// set the element at the current index
	///
	/// # Example
	/// ```
	/// use ivec::IVec;
	///
	/// let mut ivec = IVec::from(vec![0, 42, -21]);
	///
	/// assert_eq!(ivec.get(), &0);
	/// ivec.set(84);
	/// assert_eq!(ivec.get(), &84);
	/// ```
	#[inline]
	pub fn set(&mut self, t: T) {
		let i = self.get_i();
		self.vec[i] = t;
	}

	/// go to the next element and return a reference to it
	///
	/// # Example
	/// ```
	/// use ivec::IVec;
	///
	/// let mut ivec = IVec::from(vec![0, 42, -21]);
	///
	/// assert_eq!(ivec.get(), &0);
	/// assert_eq!(ivec.next(), &42);
	/// assert_eq!(ivec.next(), &-21);
	/// assert_eq!(ivec.next(), &0);
	/// ```
	#[allow(clippy::should_implement_trait)]
	#[inline]
	pub fn next(&mut self) -> &T {
		self.plus_i(1);
		&self.vec[self.i]
	}

	/// go to the previous element and return a reference to it
	///
	/// # Example
	/// ```
	/// use ivec::IVec;
	///
	/// let mut ivec = IVec::from(vec![0, 42, -21]);
	///
	/// assert_eq!(ivec.get(), &0);
	/// assert_eq!(ivec.prev(), &-21);
	/// assert_eq!(ivec.prev(), &42);
	/// assert_eq!(ivec.prev(), &0);
	/// ```
	#[inline]
	pub fn prev(&mut self) -> &T {
		self.plus_i(self.vec.len() - 1);
		&self.vec[self.i]
	}

	/// go to the element at the given index and return a reference to it
	///
	/// # Example
	/// ```
	/// use ivec::IVec;
	///
	/// let mut ivec = IVec::from(vec![0, 42, -21]);
	///
	/// assert_eq!(ivec.to(0), &0);
	/// assert_eq!(ivec.to(1), &42);
	/// assert_eq!(ivec.to(2), &-21);
	/// assert_eq!(ivec.to(3), &0);
	/// assert_eq!(ivec.to(4), &42);
	/// assert_eq!(ivec.to(5), &-21);
	/// ```
	#[inline]
	pub fn to(&mut self, i: usize) -> &T {
		self.set_i(i);
		&self.vec[self.i]
	}
}
