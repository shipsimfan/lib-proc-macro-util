use crate::ast::expressions::{BlockExpression, IfExpression};

mod parse;
mod to_static;
mod to_tokens;

/// The contents of an else statement
#[derive(Debug, Clone)]
pub enum ElseBlockExpression<'a> {
    /// This is the final else block in the expression
    Block(BlockExpression<'a>),

    /// There may be other else block beyond
    If(Box<IfExpression<'a>>),
}
