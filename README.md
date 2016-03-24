# leftpad-rs

This crate provides generic left-padding functions for strings, including both `&str` and `String`.

Import with `extern crate left_pad;`.

## [Link to the documentation](https://futile.github.io/leftpad-rs/left_pad/index.html)

Usage example:

```rust
use left_pad::{leftpad, leftpad_with};

assert_eq!(leftpad("blubb", 7), "  blubb");
assert_eq!(leftpad_with("blubb", 7, '.'), "..blubb");

let s: String = "blubb".to_owned();
assert_eq!(leftpad(s, 7), "  blubb");
```
