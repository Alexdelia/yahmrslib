// r, g, b are in the range 0..=255
// make sure that the macro check it at compile time

#[proc_macro]
pub fn rgb(token: TokenStream) -> TokenStream {
	let mut iter = token.into_iter();
	let r = match iter.next() {
		Some(TokenTree::Literal(lit)) => lit,
		_ => panic!("Expected a literal"),
	};
	let g = match iter.next() {
		Some(TokenTree::Literal(lit)) => lit,
		_ => panic!("Expected a literal"),
	};
	let b = match iter.next() {
		Some(TokenTree::Literal(lit)) => lit,
		_ => panic!("Expected a literal"),
	};
	let r = r.to_string().parse::<u8>().unwrap();
	let g = g.to_string().parse::<u8>().unwrap();
	let b = b.to_string().parse::<u8>().unwrap();
	let s = format!("\x1b[38;2;{};{};{}m", r, g, b);
	TokenStream::from_str(&s).unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_rgb() {
		let s = rgb(quote! { 255 255 255 });
		assert_eq!(s.to_string(), "\x1b[38;2;255;255;255m");
	}
}
