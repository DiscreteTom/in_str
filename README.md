# `in_str!`

[![Crates.io Version](https://img.shields.io/crates/v/in_str?style=flat-square)](https://crates.io/crates/in_str)
![license](https://img.shields.io/github/license/DiscreteTom/in_str?style=flat-square)

```rs
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
```
