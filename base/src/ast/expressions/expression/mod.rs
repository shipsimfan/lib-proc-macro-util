use crate::ast::{ExpressionWithBlock, ExpressionWithoutBlock};

mod parse;
mod to_static;
mod to_tokens;

/// A construct that evaluates to a value, and it can be used to perform computations, manipulate
/// data, or control the flow of a program
#[derive(Debug, Clone)]
pub enum Expression<'a> {
    /// The expression has a block
    WithBlock(ExpressionWithBlock<'a>),

    /// The expression has no block
    WithoutBlock(ExpressionWithoutBlock<'a>),
}
