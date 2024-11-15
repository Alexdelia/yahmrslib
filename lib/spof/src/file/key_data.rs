use crate::{ExpectedLine, FoundLine};

#[derive(Debug)]
pub struct KeyData {
	pub data: FoundLine,
	pub rule: ExpectedLine,
}

impl KeyData {
	pub fn new(data: FoundLine, rule: ExpectedLine) -> Self {
		Self { data, rule }
	}
}
