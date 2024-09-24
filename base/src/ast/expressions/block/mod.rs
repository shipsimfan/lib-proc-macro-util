use crate::ast::{ExpressionWithoutBlock, InnerAttribute, Statement};

mod default;
mod from;
mod new;
mod parse;
mod to_tokens;

/// A block expression, or block, is a control flow expression and anonymous namespace scope for
/// items and variable declarations.
#[derive(Debug, Clone)]
pub struct BlockExpression<'a> {
    /// The set of attributes affecting this block
    pub attributes: Vec<InnerAttribute<'a>>,

    /// The statements making up the block
    pub statements: Vec<Statement>,

    /// A final return value
    pub end: Option<ExpressionWithoutBlock<'a>>,
}