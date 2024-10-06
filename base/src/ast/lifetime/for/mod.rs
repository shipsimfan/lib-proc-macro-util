use crate::{ast::GenericParams, Token};

mod parse;
mod to_tokens;

/// A set of lifetime restrictions a following bound must be applicable for
#[derive(Debug, Clone)]
pub struct ForLifetimes<'a> {
    /// The for marking the beginning of the set
    r#for: Token![for],

    /// The generics restricting the following bound
    generics: GenericParams<'a>,
}
