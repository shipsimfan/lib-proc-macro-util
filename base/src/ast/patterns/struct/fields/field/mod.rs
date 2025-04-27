use crate::ast::OuterAttribute;

mod name;

mod parse;
mod to_static;
mod to_tokens;

pub use name::StructPatternFieldName;

/// A field in struct pattern
#[derive(Debug, Clone)]
pub struct StructPatternField<'a> {
    /// The attributes affecting this field
    pub attributes: Vec<OuterAttribute<'a>>,

    /// The name of the field and the pattern to match
    pub name: StructPatternFieldName<'a>,
}
