# `in_str!`

![license](https://img.shields.io/github/license/DiscreteTom/in_str?style=flat-square)
[![Crates.io Version](https://img.shields.io/crates/v/in_str?style=flat-square)](https://crates.io/crates/in_str)
[![docs.rs](https://img.shields.io/docsrs/in_str?style=flat-square)](https://docs.rs/in_str/)

A procedural macro to generate a closure that checks if a character is in the provided literal string.

## Installation

```bash
cargo add in_str
```

## Usage

```rust
use in_str::in_str;

let _ = in_str!("abc");
// equals to
let _ = |c: char| matches!(c, 'a' | 'b' | 'c');
// usually faster than
let _ = |c: char| "abc".contains(c);

// escape will be handled automatically
let _ = in_str!("\n\u{10ffff}");
// equals to
let _ = |c: char| matches!(c, '\n' | '\u{10ffff}');

// also works with byte strings
let _ = in_str!(b"abc");
// equals to
let _ = |c: u8| matches!(c, b'a' | b'b' | b'c');
// escape will be handled automatically
let _ = in_str!(b"\n\xff");
// equals to
let _ = |c: u8| matches!(c, b'\n' | 0xff);
```

## [Documentation](https://docs.rs/in_str/)

## [CHANGELOG](./CHANGELOG.md)
