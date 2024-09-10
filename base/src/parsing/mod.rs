use crate::tokens::TokenTree;

mod error;
mod parse;
mod parser;

pub use error::{Error, ErrorMessage, Result};
pub use parse::Parse;
pub use parser::Parser;

/// Attempts to parse an object from a [`TokenStream`]
///
/// ## Parameters
///  * `input` - The [`proc_macro::TokenStream`] to parse
///  * `full` - If true, this function will verify the full stream was parsed or it will return an
///             error.
///
/// ## Return Value
/// Returns the object parsed from `input` on success.
pub fn parse<'a, T: Parse<'a>>(tokens: &'a [TokenTree], full: bool) -> Result<T> {
    let mut parser = Parser::new(&tokens);
    let result = parser.parse()?;
    if !full || parser.empty() {
        Ok(result)
    } else {
        Err(Error::new("expected the end"))
    }
}
