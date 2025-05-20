use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, Parser};

struct RGBInput {
	r: u8,
	g: u8,
	b: u8,
}

impl syn::parse::Parse for RGBInput {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		fn parse_u8(input: syn::parse::ParseStream, comma: bool) -> syn::Result<u8> {
			let Ok(lit) = input.parse::<syn::LitInt>() else {
				return Err(syn::Error::new_spanned(
					input.to_string(),
					"expected integer literal",
				));
			};
			let Ok(val) = lit.base10_parse::<u8>() else {
				return Err(syn::Error::new_spanned(
					lit,
					"expected u8 literal (0..=255)",
				));
			};

			if comma && input.parse::<syn::Token![,]>().is_err() {
				return Err(syn::Error::new_spanned(
					input.to_string(),
					"expected 3 comma-separated integers",
				));
			}

			Ok(val)
		}
		let r = parse_u8(input, true)?;
		let g = parse_u8(input, true)?;
		let b = parse_u8(input, false)?;
		Ok(RGBInput { r, g, b })
	}
}

#[proc_macro]
pub fn rgb(input: TokenStream) -> TokenStream {
	let RGBInput { r, g, b } = match RGBInput::parse.parse(input) {
		Ok(rgb) => rgb,
		Err(e) => return e.to_compile_error().into(),
	};

	quote!({ ansi::__formatcp!("\x1b[38;2;{};{};{}m", #r, #g, #b) }).into()
}

#[proc_macro]
pub fn rgb_bg(input: TokenStream) -> TokenStream {
	let RGBInput { r, g, b } = match RGBInput::parse.parse(input) {
		Ok(rgb) => rgb,
		Err(e) => return e.to_compile_error().into(),
	};

	quote!({ ansi::__formatcp!("\x1b[48;2;{};{};{}m", #r, #g, #b) }).into()
}

#[proc_macro]
pub fn c8bit(input: TokenStream) -> TokenStream {
	let n = syn::LitInt::parse
		.parse(input)
		.unwrap()
		.base10_parse::<u8>()
		.unwrap();
	quote!({ ansi::__formatcp!("\x1b[38;5;{}m", #n) }).into()
}

#[proc_macro]
pub fn c8bit_bg(input: TokenStream) -> TokenStream {
	let n = syn::LitInt::parse
		.parse(input)
		.unwrap()
		.base10_parse::<u8>()
		.unwrap();
	quote!({ ansi::__formatcp!("\x1b[48;5;{}m", #n) }).into()
}

struct HexInput {
	r: u8,
	g: u8,
	b: u8,
}

impl HexInput {
	fn parse(input: TokenStream) -> syn::Result<Self> {
		let mut s = input.to_string().to_lowercase();
		s.retain(|c| !c.is_whitespace() && c != '#' && c != '"' && c != '\'');
		if s.starts_with("0x") {
			s = s[2..].to_string();
		}

		if s.len() != 6 || !s.chars().all(|c| c.is_ascii_hexdigit()) {
			let mut e = format!(
            	"expected hex color (e.g. \x1b[32m#ff00ff\x1b[39m), got \x1b[31m{input}\x1b[39m -> \x1b[31;3m{s}\x1b[39;23m"
        	);
			if s.len() != 6 {
				e.push_str(
					format!(
						"\nexpected \x1b[32m6\x1b[39m chars, got \x1b[31m{}\x1b[39m",
						s.len()
					)
					.as_str(),
				);
			}
			if !s.chars().all(|c| c.is_ascii_hexdigit()) {
				let mut cs = String::new();
				let mut bits = String::new();

				for c in s.chars() {
					if !c.is_ascii_hexdigit() {
						cs.push_str(format!("\x1b[31m{c}\x1b[39m").as_str());
						bits.push_str("\x1b[31m^\x1b[39m");
					} else {
						cs.push_str(format!("\x1b[32m{c}\x1b[39m").as_str());
						bits.push(' ');
					}
				}

				e.push_str(
					format!("\n{cs}\n{bits} -> are \x1b[33mnot hex\x1b[39m digits").as_str(),
				);
			}

			return Err(syn::Error::new_spanned(input.to_string(), e));
		}

		let r = u8::from_str_radix(&s[0..2], 16).unwrap();
		let g = u8::from_str_radix(&s[2..4], 16).unwrap();
		let b = u8::from_str_radix(&s[4..6], 16).unwrap();

		Ok(HexInput { r, g, b })
	}
}

#[proc_macro]
pub fn hex(input: TokenStream) -> TokenStream {
	let HexInput { r, g, b } = match HexInput::parse(input) {
		Ok(hex) => hex,
		Err(e) => return e.to_compile_error().into(),
	};

	quote!({ ansi::__formatcp!("\x1b[38;2;{};{};{}m", #r, #g, #b) }).into()
}

#[proc_macro]
pub fn hex_bg(input: TokenStream) -> TokenStream {
	let HexInput { r, g, b } = match HexInput::parse(input) {
		Ok(hex) => hex,
		Err(e) => return e.to_compile_error().into(),
	};

	quote!({ ansi::__formatcp!("\x1b[48;2;{};{};{}m", #r, #g, #b) }).into()
}

struct HSLInput {
	h: f32,
	s: f32,
	l: f32,
}

impl HSLInput {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		fn parse_float(input: syn::parse::ParseStream, comma: bool, max: f32) -> syn::Result<f32> {
			let Ok(lit) = input.parse::<syn::LitFloat>() else {
				return Err(syn::Error::new_spanned(
					input.to_string(),
					"expected f32 literal",
				));
			};

			let expected_msg = || format!("expected f32 literal (0..={max})");

			let Ok(val) = lit.base10_parse::<f32>() else {
				return Err(syn::Error::new_spanned(lit, expected_msg()));
			};
			if val < 0.0 || val > max {
				return Err(syn::Error::new_spanned(lit, expected_msg()));
			}

			if comma && input.parse::<syn::Token![,]>().is_err() {
				return Err(syn::Error::new_spanned(
					input.to_string(),
					"expected 3 comma-separated f32",
				));
			}

			Ok(val)
		}

		let h = parse_float(input, true, 360.0)?;
		let s = parse_float(input, true, 100.0)?;
		let l = parse_float(input, false, 100.0)?;
		Ok(HSLInput { h, s, l })
	}
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
	let q = if l < 0.5 {
		l * (1.0 + s)
	} else {
		l + s - l * s
	};
	let p = 2.0 * l - q;

	let r = hue_to_rgb(p, q, h + 1.0 / 3.0);
	let g = hue_to_rgb(p, q, h);
	let b = hue_to_rgb(p, q, h - 1.0 / 3.0);

	(
		(r * 255.0).round() as u8,
		(g * 255.0).round() as u8,
		(b * 255.0).round() as u8,
	)
}

fn hue_to_rgb(p: f32, q: f32, t: f32) -> f32 {
	let t = if t < 0.0 {
		t + 1.0
	} else if t > 1.0 {
		t - 1.0
	} else {
		t
	};

	if t < 1.0 / 6.0 {
		p + (q - p) * 6.0 * t
	} else if t < 1.0 / 2.0 {
		q
	} else if t < 2.0 / 3.0 {
		p + (q - p) * (2.0 / 3.0 - t) * 6.0
	} else {
		p
	}
}

#[proc_macro]
pub fn hsl(input: TokenStream) -> TokenStream {
	let HSLInput { h, s, l } = match HSLInput::parse.parse(input) {
		Ok(hsl) => hsl,
		Err(e) => return e.to_compile_error().into(),
	};
	let (r, g, b) = hsl_to_rgb(h / 360.0, s / 100.0, l / 100.0);

	quote!({ ansi::__formatcp!("\x1b[38;2;{};{};{}m", #r, #g, #b) }).into()
}

#[proc_macro]
pub fn hsl_bg(input: TokenStream) -> TokenStream {
	let HSLInput { h, s, l } = match HSLInput::parse.parse(input) {
		Ok(hsl) => hsl,
		Err(e) => return e.to_compile_error().into(),
	};
	let (r, g, b) = hsl_to_rgb(h / 360.0, s / 100.0, l / 100.0);

	quote!({ ansi::__formatcp!("\x1b[48;2;{};{};{}m", #r, #g, #b) }).into()
}
