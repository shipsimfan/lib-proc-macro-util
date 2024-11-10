use crate::{supported_languages::*, tokens::TokenTree};
use i18n_translation::m;

mod collect_token_stream;
mod error;
mod parse;
mod parser;

pub use collect_token_stream::collect_token_stream;
pub use error::{Error, ErrorMessage, Result};
pub use parse::Parse;
pub use parser::Parser;

i18n_translation::message_key!(EXPECTED_END [
    EN => { "expected the end of the macro" },
    FR => { "la fin de la macro était attendue" },
    ZH => { "预期的宏结束" },
]);

/// Attempts to parse an object from a [`proc_macro::TokenStream`]
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
        Err(Error::new(m!(EXPECTED_END)))
    }
}
