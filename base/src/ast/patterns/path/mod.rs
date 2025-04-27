use crate::ast::expressions::PathExpression;

mod parse;
mod to_static;
mod to_tokens;

/// Path patterns are patterns that refer either to constant values or to structs or enum variants
/// that have no fields.
#[derive(Debug, Clone)]
pub struct PathPattern<'a> {
    /// The path itself
    pub path: PathExpression<'a>,
}
