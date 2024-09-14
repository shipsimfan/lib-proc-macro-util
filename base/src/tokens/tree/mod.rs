use crate::tokens::{Group, Identifier, Literal, Punctuation};

mod from;
mod into;
mod parse;
mod to_tokens;

/// One token or tree
#[derive(Debug, Clone)]
pub enum TokenTree {
    /// A delimited group of tokens
    Group(Group),

    /// An identifier
    Identifier(Identifier),

    /// A string or numeric literal
    Literal(Literal),

    /// A single character of punctuation
    Punctuation(Punctuation),
}
