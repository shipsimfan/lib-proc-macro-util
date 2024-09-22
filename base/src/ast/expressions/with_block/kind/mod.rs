use crate::ast::expressions::BlockExpression;

mod from;
mod new;
mod parse;
mod to_tokens;

/// A specific type of expression that has a block
#[derive(Debug, Clone)]
pub enum ExpressionWithBlockKind<'a> {
    /// An expression made up of only a block
    Block(BlockExpression<'a>),
}
