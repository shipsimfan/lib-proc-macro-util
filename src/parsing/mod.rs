// rustdoc imports
#[allow(unused_imports)]
use proc_macro::TokenStream;

mod buffer;
mod error;
mod parse;
mod parser;

pub use error::{Error, Result};
pub use parse::Parse;
pub use parser::Parser;

pub(crate) use buffer::TokenBuffer;

/// Attempts to parse an object from a [`TokenStream`]
///
/// ## Parameters
///  * `input` - The [`TokenStream`] to parse
///
/// ## Return Value
/// Returns the object parsed from `input` on success.
pub fn parse<T: Parse>(input: proc_macro::TokenStream) -> Result<T> {
    let token_buffer: TokenBuffer = input.into();
    let mut parser: Parser = (&token_buffer).into();
    parser.parse()
}
