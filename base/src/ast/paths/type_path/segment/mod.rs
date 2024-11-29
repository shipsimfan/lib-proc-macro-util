use crate::{ast::PathIdentSegment, Token};

mod generics;

mod parse;
mod to_static;
mod to_tokens;

pub use generics::TypePathSegmentGenerics;

/// A single segment of a type path
#[derive(Debug, Clone)]
pub struct TypePathSegment<'a> {
    /// The identifier defining this segment
    pub ident: PathIdentSegment,

    /// The generics modifying this segment
    pub generics: Option<(Option<Token![::]>, TypePathSegmentGenerics<'a>)>,
}
