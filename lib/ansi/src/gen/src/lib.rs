use std::fmt::format;

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
        let r = input.parse::<syn::LitInt>()?.base10_parse::<u8>()?;
        input.parse::<syn::Token![,]>()?;
        let g = input.parse::<syn::LitInt>()?.base10_parse::<u8>()?;
        input.parse::<syn::Token![,]>()?;
        let b = input.parse::<syn::LitInt>()?.base10_parse::<u8>()?;
        Ok(RGBInput { r, g, b })
    }
}

#[proc_macro]
pub fn rgb(input: TokenStream) -> TokenStream {
    let RGBInput { r, g, b } = RGBInput::parse.parse(input).unwrap();
    quote!({ format!("\x1b[38;2;{};{};{}m", #r, #g, #b) }).into()
}

#[proc_macro]
pub fn rgb_bg(input: TokenStream) -> TokenStream {
    let RGBInput { r, g, b } = RGBInput::parse.parse(input).unwrap();
    quote!({ format!("\x1b[48;2;{};{};{}m", #r, #g, #b) }).into()
}

#[proc_macro]
pub fn c8bit(input: TokenStream) -> TokenStream {
    let n = syn::LitInt::parse
        .parse(input)
        .unwrap()
        .base10_parse::<u8>()
        .unwrap();
    quote!({ format!("\x1b[38;5;{}m", #n) }).into()
}

#[proc_macro]
pub fn c8bit_bg(input: TokenStream) -> TokenStream {
    let n = syn::LitInt::parse
        .parse(input)
        .unwrap()
        .base10_parse::<u8>()
        .unwrap();
    quote!({ format!("\x1b[48;5;{}m", #n) }).into()
}

#[proc_macro]
pub fn hex(input: TokenStream) -> TokenStream {
    // parse hex of format: "#FF00FF" or "FF00FF" or "ff00ff" or #FF00FF or #ff00ff or ff00ff
    // get TokenStream as raw string
    let mut s = input.to_string().to_lowercase();
    s.retain(|c| !c.is_whitespace() || c == '#' || c == '"' || c == '\'');
    if s.starts_with("0x") {
        s = s[2..].to_string();
    }

    if s.len() != 6 || !s.chars().all(|c| c.is_ascii_hexdigit()) {
        let mut e = format!("expected hex color (e.g. #ff00ff), got {s}");
        if s.len() != 6 {
            e.push_str(format!("\ngot {} chars, expected 6", s.len()).as_str());
        }
        if !s.chars().all(|c| c.is_ascii_hexdigit()) {
            let mut bits = String::new();

            for c in s.chars() {
                if !c.is_ascii_hexdigit() {
                    bits.push(c);
                } else {
                    bits.push_str("_");
                }
            }

            e.push_str(format!("\n{bits} are not hex digits").as_str());
        }

        return syn::Error::new(syn::spanned::Spanned::span(&input.to_string()), e)
            .to_compile_error()
            .into();
    }

    quote!(#s).into()

    // quote!({ format!("\x1b[38;2;{};{};{}m", #r, #g, #b) }).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb() {
        let s = rgb(quote! { 42, 255, 0 }.into()).to_string();
        assert_eq!(s, "\"\x1b[38;2;42;255;0m\"");
    }

    #[test]
    fn test_rgb_bg() {
        let s = rgb_bg(quote! { 42, 255, 0 }.into()).to_string();
        assert_eq!(s, "\"\x1b[48;2;42;255;0m\"");
    }

    #[test]
    fn test_c8bit() {
        let s = c8bit(quote! { 42 }.into()).to_string();
        assert_eq!(s, "\"\x1b[38;5;42m\"");
    }

    #[test]
    fn test_c8bit_bg() {
        let s = c8bit_bg(quote! { 42 }.into()).to_string();
        assert_eq!(s, "\"\x1b[48;5;42m\"");
    }
}
