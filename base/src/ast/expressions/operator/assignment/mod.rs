use crate::{ast::Expression, Token};

mod parse;
mod to_static;
mod to_tokens;

/// An expressions which sets the contents of a variable
#[derive(Debug, Clone)]
pub struct AssignmentExpression<'a> {
    /// The left side of the operation
    pub left: Box<Expression<'a>>,

    /// The equals indicating this is an assignment
    pub equals: Token![=],

    /// The right side of the operation
    pub right: Box<Expression<'a>>,
}
