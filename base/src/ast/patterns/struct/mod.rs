use crate::ast::PathInExpression;

mod elements;
mod et_cetera;
mod fields;

mod parse;
mod to_static;
mod to_tokens;

pub use elements::StructPatternElements;
pub use et_cetera::StructPatternEtCetera;
pub use fields::{StructPatternField, StructPatternFieldName, StructPatternFields};

/// Struct patterns match struct, enum, and union values that match all criteria defined by its
/// subpatterns.
#[derive(Debug, Clone)]
pub struct StructPattern<'a> {
    /// The path to the struct
    pub path: PathInExpression<'a>,

    /// The elements to be matched in the struct
    pub elements: Option<StructPatternElements<'a>>,
}
