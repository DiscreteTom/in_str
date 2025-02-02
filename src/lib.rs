#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse, LitByteStr, LitStr};

/// Generate a closure that checks if a character is in the provided literal string
/// using the [`matches!`] macro.
/// This is usually faster than using [`str::contains`].
/// # Examples
/// ```
/// use in_str::in_str;
///
/// let _ = in_str!("abc");
/// // equals to
/// let _ = |c: char| matches!(c, 'a' | 'b' | 'c');
/// // usually faster than
/// let _ = |c: char| "abc".contains(c);
///
/// // escape will be handled automatically
/// let _ = in_str!("\n\u{10ffff}");
/// // equals to
/// let _ = |c: char| matches!(c, '\n' | '\u{10ffff}');
///
/// // also works with byte strings
/// let _ = in_str!(b"abc");
/// // equals to
/// let _ = |c: u8| matches!(c, b'a' | b'b' | b'c');
/// // escape will be handled automatically
/// let _ = in_str!(b"\n\xff");
/// // equals to
/// let _ = |c: u8| matches!(c, b'\n' | 0xff);
/// ```
#[proc_macro]
pub fn in_str(item: TokenStream) -> TokenStream {
  let mut gen = Vec::new();
  if let Ok(s) = parse::<LitStr>(item.clone()) {
    for c in s.value().chars() {
      gen.push(quote! { #c });
    }
    return quote! { |c: char| matches!(c, #(#gen)|*) }.into();
  }
  if let Ok(s) = parse::<LitByteStr>(item) {
    for c in s.value() {
      gen.push(quote! { #c });
    }
    return quote! { |c: u8| matches!(c, #(#gen)|*) }.into();
  }
  panic!("Expected a string or byte string literal");
}
