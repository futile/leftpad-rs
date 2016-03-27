/*!
This crate provides left-padding for strings (including both `&str` and `String`).

Import with `extern crate left_pad;`.

Usage example:

```
use left_pad::{leftpad, leftpad_with};

assert_eq!(leftpad("blubb", 7), "  blubb");
assert_eq!(leftpad_with("blubb", 7, '.'), "..blubb");

let s: String = "blubb".to_owned();
assert_eq!(leftpad(s, 7), "  blubb");
```
*/

#![deny(missing_docs)]

use std::borrow::Cow;
use std::borrow::Borrow;

/// Pads a string to the given length `len` by inserting the character `pad_char` from the left.
///
/// If the given string has a length longer or equal to the desired length, it will be
/// returned as-is.
///
/// # Examples
///
/// ```
/// use left_pad::leftpad_with;
///
/// assert_eq!(leftpad_with("blubb", 7, ' '), "  blubb");
/// assert_eq!(leftpad_with("blubb", 7, '.'), "..blubb");
///
/// assert_eq!(leftpad_with("blubb", 5, ' '), "blubb");
/// assert_eq!(leftpad_with("blubb", 3, ' '), "blubb");
/// ```
pub fn leftpad_with<'a, S>(string: S, len: usize, pad_char: char) -> Cow<'a, str>
    where S: Into<Cow<'a, str>>
{
    let cow = string.into();

    if !(len > cow.len()) {
        return cow;
    }

    let to_pad = len - cow.len();
    let mut padded = String::with_capacity(cow.len() + to_pad);

    for _ in 0..to_pad {
        padded.push(pad_char);
    }

    padded.push_str(cow.borrow());

    padded.into()
}

/// Pads a string to the given length `len` by inserting whitespaces from the left.
///
/// If the given string has a length longer or equal to the desired length, it will be
/// returned as-is.
///
/// This function is equal to calling `leftpad_with(string, len, ' ')`.
///
/// # Examples
///
/// ```
/// use left_pad::{leftpad,leftpad_with};
///
/// assert_eq!(leftpad("blubb", 7), "  blubb");
///
/// assert_eq!(leftpad("blubb", 5), "blubb");
/// assert_eq!(leftpad("blubb", 3), "blubb");
///
/// assert_eq!(leftpad("blubb", 7), leftpad_with("blubb", 7, ' '));
/// ```
pub fn leftpad<'a, S>(string: S, len: usize) -> Cow<'a, str>
    where S: Into<Cow<'a, str>>
{
    leftpad_with(string, len, ' ')
}
