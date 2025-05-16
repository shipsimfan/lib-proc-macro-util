use crate::ast::Expression;

mod to_static;
mod to_tokens;

/// Access an element in an item at a given index
#[derive(Debug, Clone)]
pub struct IndexExpression<'a> {
    /// The expression being indexed
    pub expression: Box<Expression<'a>>,

    /// The expression evaluating to the index
    pub index: Box<Expression<'a>>,
}
