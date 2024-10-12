use crate::{ast::TraitBound, Token};

mod parse;
mod to_tokens;

/// A reference to an unnamed type which only implements one trait
#[derive(Debug, Clone)]
pub struct ImplTraitTypeOneBound<'a> {
    /// The `impl` token marking this type
    pub r#impl: Token![impl],

    /// The bound this type must conform to
    pub bound: TraitBound<'a>,
}
