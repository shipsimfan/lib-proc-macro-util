use crate::{tokens::Identifier, Token};

mod display;
mod from;
mod new;
mod parse;
mod to_tokens;

/// A lifetime denoting scope of a variable
#[derive(Debug, Clone)]
pub enum Lifetime<'a> {
    /// The lifetime is named and the name is borrowed
    Identifier(Token!['_], &'a Identifier),

    /// The lifetime is named and the name is owned
    IdentifierOwned(Token!['_], Identifier),

    /// The lifetime is anonymous
    Underscore(Token!['_], Token![_]),
}
