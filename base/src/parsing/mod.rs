// rustdoc imports
#[allow(unused_imports)]
use proc_macro::TokenStream;

mod buffer;
mod error;
mod parse;
mod parser;

pub use buffer::TokenBuffer;
pub use error::{Error, ErrorMessage, Result};
pub use parse::Parse;
pub use parser::Parser;

/// Attempts to parse an object from a [`TokenBuffer`]
///
/// ## Parameters
///  * `input` - The [`TokenBuffer`] to parse
///  * `full` - If true, this function will verify the full stream was parsed or it will return an
///             error.
///
/// ## Return Value
/// Returns the object parsed from `input` on success.
pub fn parse<'a, T: Parse<'a>>(input: &'a TokenBuffer, full: bool) -> Result<T> {
    let mut parser: Parser = (input).into();
    let result = parser.parse()?;
    if !full || parser.empty() {
        Ok(result)
    } else {
        Err(Error::new("expected the end"))
    }
}
