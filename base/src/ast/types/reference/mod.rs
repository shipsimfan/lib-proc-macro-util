use crate::{
    ast::{Lifetime, TypeNoBounds},
    Token,
};

mod parse;
mod to_tokens;

/// A reference to another type
#[derive(Debug, Clone)]
pub struct ReferenceType<'a> {
    /// The `&` marking this as a reference type
    pub reference: Token![&],

    /// A lifetime for this reference
    pub lifetime: Option<Lifetime<'a>>,

    /// Is the reference mutable?
    pub r#mut: Option<Token![mut]>,

    /// The type being referenced
    pub r#type: Box<TypeNoBounds<'a>>,
}
