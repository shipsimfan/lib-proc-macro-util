use crate::ast::expressions::IteratorLoopExpression;

mod parse;
mod to_static;
mod to_tokens;

/// The kind of loop a loop expression is
#[derive(Debug, Clone)]
pub enum LoopExpressionKind<'a> {
    /// The expression loops over the results of an iterator
    Iterator(IteratorLoopExpression<'a>),
}
