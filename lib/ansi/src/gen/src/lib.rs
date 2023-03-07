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
}
