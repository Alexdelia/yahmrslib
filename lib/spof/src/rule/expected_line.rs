use crate::{Format, Keyword, Occurrence};

use ansi::abbrev::{B, D, G, M, R, Y};
use hmerr::{Result, pfe, ple, pwe};

#[derive(Debug)]
pub struct ExpectedLine {
	pub k: Keyword,
	pub format: Format,
	pub occurrence: Occurrence,
}

impl ExpectedLine {
	pub fn new(k: Keyword, format: Format, occurrence: Occurrence) -> Self {
		Self {
			k,
			format,
			occurrence,
		}
	}

	pub fn check<T>(
		&self,
		file_name: &str,
		line: String,
		token: &[T],
		line_index: usize,
	) -> Result<()> {
		match self.format.check(token) {
			Ok(_) => Ok(()),
			Err((expected, got)) => {
				let line_len = line.len();
				pfe!(
					format!("expected {B}{G}{expected}{D} token after {B}{Y}{keyword}{D}, got {B}{R}{got}{D}",
						keyword=self.k.keyword,
					),
					h:self.help(),
					f:file_name,
					l:ple!(line, i:line_index, w:pwe!((0, line_len)))
				)?
			}
		}
	}

	pub fn help(&self) -> String {
		format!(
			"{B}{Y}{keyword}{D} define {desc}
the line must follow the format: `{B}{keyword} {M}{format}{D}`",
			keyword = self.k.keyword,
			desc = self.k.desc,
			format = self.format.token,
		)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::ExpectedSize;

	#[test]
	fn test_expected_line() {
		let el = ExpectedLine::new(
			Keyword::new("keyword", "desc"),
			Format::new("format", ExpectedSize::Fixed),
			Occurrence::OneOrMore,
		);
		let content = vec![
			"keyword format".to_string(),
			"keyword format wrong".to_string(),
		];

		let mut split: Vec<String> = content[0]
			.split_whitespace()
			.map(|s| s.to_string())
			.collect();
		let _keyword = split.remove(0);
		assert!(el.check("file_name", content[0].clone(), &split, 0).is_ok());

		let mut split: Vec<String> = content[1]
			.split_whitespace()
			.map(|s| s.to_string())
			.collect();
		let _keyword = split.remove(0);
		assert!(
			el.check("file_name", content[1].clone(), &split, 1)
				.is_err()
		);
	}
}
