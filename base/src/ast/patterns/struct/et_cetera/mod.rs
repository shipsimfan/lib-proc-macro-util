use crate::{ast::OuterAttribute, Token};

mod parse;
mod to_static;
mod to_tokens;

/// A marker for all remaining fields in a struct
#[derive(Debug, Clone)]
pub struct StructPatternEtCetera<'a> {
    /// The attributes affecting the rest of the fields
    pub attributes: Vec<OuterAttribute<'a>>,

    /// The dots marking this as an et cetera
    pub dots: Token![..],
}
