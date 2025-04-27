use crate::Token;

mod field;

mod parse;
mod to_static;
mod to_tokens;

pub use field::{StructPatternField, StructPatternFieldName};

/// The defined fields in a struct pattern
#[derive(Debug, Clone)]
pub struct StructPatternFields<'a> {
    /// The first defined field
    pub first: StructPatternField<'a>,

    /// The remaining fields
    pub remaining: Vec<(Token![,], StructPatternField<'a>)>,
}
