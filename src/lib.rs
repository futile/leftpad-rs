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

/// Pads a string to the given number of chars by inserting the character `pad_char` from the left.
///
/// If the given string has more than or exactly the desired number of codepoints, it will be
/// returned as-is.
///
/// Codepoints are not graphemes, so the result might always not be what a human would expect.
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
pub fn leftpad_with<'a, S>(string: S, codepoints: usize, pad_char: char) -> Cow<'a, str>
    where S: Into<Cow<'a, str>>
{
    let cow = string.into();

    let cow_codepoints = {
        let s: &str = cow.borrow();
        s.len() - s.bytes()
                   .filter(|b| (b >> 6) == 0b10u8 )
                   .count()
    };
    if codepoints <= cow_codepoints {
        return cow;
    }

    let to_pad = codepoints - cow_codepoints;
    let mut padded = String::with_capacity(cow.len() + to_pad);

    for _ in 0..to_pad {
        padded.push(pad_char);
    }

    padded.push_str(cow.borrow());

    padded.into()
}

/// Pads a string to the given number of chars by inserting spaces from the left.
///
/// If the given string has more than or exactly the desired number of codepoints, it will be
/// returned as-is.
///
/// Codepoints are not graphemes, so the result might not always be what a human would expect.
///
/// This function is equal to calling `leftpad_with(string, codepoints, ' ')`.
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
pub fn leftpad<'a, S>(string: S, codepoints: usize) -> Cow<'a, str>
    where S: Into<Cow<'a, str>>
{
    leftpad_with(string, codepoints, ' ')
}
