use crate::{
    ast::{ForLifetimes, TypePath},
    Token,
};

mod parse;
mod to_tokens;

/// A bound restricting a type to a trait
#[derive(Debug, Clone)]
pub struct TraitBound<'a> {
    /// Is this bound surrounded by parentheses
    pub delimited: bool,

    /// A question mark indicating this trait bound is optional
    pub question: Option<Token![?]>,

    /// Lifetimes this trait bound must be valid over
    pub for_lifetimes: Option<ForLifetimes<'a>>,

    /// The path to the trait
    pub path: TypePath<'a>,
}
