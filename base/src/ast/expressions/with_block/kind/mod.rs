use crate::ast::expressions::{BlockExpression, UnsafeBlockExpression};

mod from;
mod new;
mod parse;
mod to_static;
mod to_tokens;

/// A specific type of expression that has a block
#[derive(Debug, Clone)]
pub enum ExpressionWithBlockKind<'a> {
    /// An expression made up of only a block
    Block(BlockExpression<'a>),

    /// A block of unsafe code
    Unsafe(UnsafeBlockExpression<'a>),
}
