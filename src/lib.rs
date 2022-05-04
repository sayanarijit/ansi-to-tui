#![warn(missing_docs)]
//! Parses a `Vec<u8>` as an byte sequence with ansi colors to
//! [`tui::text::Text`][Text].  
//!
//! Invalid ansi colors / sequences will be ignored.  
//!
//!
//! Supported features
//! - UTF-8 using `String::from_utf8` or [`simdutf8`][simdutf8].
//! - Most stuff like **Bold** / *Italic* / <u>Underline</u> / ~~Strikethrough~~.
//! - Supports 4-bit color palletes.
//! - Supports 8-bit color.
//! - Supports True color ( RGB / 24-bit color ).
//!
//!
//! ## Example
//! The argument to the function `ansi_to_text` implements `IntoIterator` so it will be consumed on
//! use.
//! ```rust
//! use ansi_to_tui_forked::ansi_to_text;
//! let bytes = b"\x1b[38;2;225;192;203mAAAAA\x1b[0m".to_owned().to_vec();
//! let text = ansi_to_text(bytes).unwrap();
//! ```
//! Example parsing from a file.
//! ```rust
//! use ansi_to_tui_forked::ansi_to_text;
//! use std::io::Read;
//!
//! let file = std::fs::File::open("text.ascii");
//! let mut buffer: Vec<u8> = Vec::new();
//! let text = ansi_to_text(buffer);
//! ```
//!
//! If you want to use [`simdutf8`][simdutf8] instead of `String::from_utf8()`  
//! for parsing UTF-8 then enable optional feature `simd`  
//!  
//! [Text]: https://docs.rs/tui/0.15.0/tui/text/struct.Text.html
//! [ansi-to-tui]: https://github.com/uttarayan21/ansi-to-tui
//! [simdutf8]: https://github.com/rusticstuff/simdutf8

mod ansi;
mod code;
mod error;
mod stack;

pub use ansi::{ansi_to_text, ansi_to_text_override_style};
pub use code::AnsiCode;
pub use error::Error;
