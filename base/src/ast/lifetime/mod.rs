use std::borrow::Cow;

use crate::{tokens::Identifier, Token};

mod bounds;
mod r#for;

mod display;
mod from;
mod new;
mod parse;
mod to_tokens;

pub use bounds::LifetimeBounds;
pub use r#for::ForLifetimes;

/// A lifetime denoting scope of a variable
#[derive(Debug, Clone)]
pub enum Lifetime<'a> {
    /// The lifetime is named
    Identifier(Token!['_], Cow<'a, Identifier>),

    /// The lifetime is anonymous
    Underscore(Token!['_], Token![_]),
}
