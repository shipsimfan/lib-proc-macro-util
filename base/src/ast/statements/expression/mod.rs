use crate::{
    ast::{ExpressionWithBlock, ExpressionWithoutBlock},
    Token,
};

mod parse;
mod to_static;
mod to_tokens;

/// An [`ExpressionStatement`] is one that evaluates an expression and ignores its result.
#[derive(Debug, Clone)]
pub enum ExpressionStatement<'a> {
    /// The expression has a block
    WithBlock(ExpressionWithBlock<'a>, Option<Token![;]>),

    /// The expression has no block
    WithoutBlock(ExpressionWithoutBlock<'a>, Token![;]),
}
