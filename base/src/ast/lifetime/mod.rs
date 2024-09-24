use crate::{tokens::Identifier, Token};

mod bounds;

mod display;
mod from;
mod new;
mod parse;
mod to_tokens;

pub use bounds::LifetimeBounds;

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
