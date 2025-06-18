use crate::tokens::TokenTree;

mod collect_token_stream;
mod parse;
mod parser;

pub use collect_token_stream::collect_token_stream;
pub use parse::Parse;
pub use parser::Parser;

/// A result from parsing tokens
pub type Result<T> = core::result::Result<T, ()>;

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
        parser.span().error("unexpected token").emit();
        Err(())
    }
}
