use proc_macro_util_base::tokens::Identifier;
use token::Token;

mod token;

mod parse;
mod to_tokens;

/// The implementation for the [`to_tokens`](crate::to_tokens) macro
#[derive(Debug, Clone)]
pub struct ToTokens<'a> {
    /// The name of the generator to use
    generator: &'a Identifier,

    /// The tokens to generate
    tokens: Vec<Token<'a>>,
}
