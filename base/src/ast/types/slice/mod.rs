use crate::ast::Type;

mod parse;
mod to_tokens;

/// A dynamically sized homogenous sequence
#[derive(Debug, Clone)]
pub struct SliceType<'a> {
    /// The type of the contained elements
    pub r#type: Box<Type<'a>>,
}
