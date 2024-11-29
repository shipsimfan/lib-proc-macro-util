use crate::{ast::TypeParamBounds, Token};

mod parse;
mod to_static;
mod to_tokens;

/// A reference to an unnamed concrete type implementing one or more traits
#[derive(Debug, Clone)]
pub struct ImplTraitType<'a> {
    /// The `impl` token marking this type
    pub r#impl: Token![impl],

    /// The bounds this type must conform to
    pub bounds: TypeParamBounds<'a>,
}
