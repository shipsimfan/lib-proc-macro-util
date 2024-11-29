use crate::{ast::TraitBound, Token};

mod parse;
mod to_static;
mod to_tokens;

/// An opaque value of another type that implements a trait
#[derive(Debug, Clone)]
pub struct TraitObjectTypeOneBound<'a> {
    /// The `dyn` token introducing this element
    pub r#dyn: Option<Token![dyn]>,

    /// The trait the type must implement
    pub bound: TraitBound<'a>,
}
