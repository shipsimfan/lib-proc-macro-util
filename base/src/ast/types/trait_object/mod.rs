use crate::{ast::TypeParamBounds, Token};

mod parse;
mod to_tokens;

/// An opaque value of another type that implements a set of traits
#[derive(Debug, Clone)]
pub struct TraitObjectType<'a> {
    /// The `dyn` token introducing this element
    pub r#dyn: Option<Token![dyn]>,

    /// The set of traits which the type must implement
    pub bounds: TypeParamBounds<'a>,
}
