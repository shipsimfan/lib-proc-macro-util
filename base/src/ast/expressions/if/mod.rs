use crate::{
    ast::{expressions::BlockExpression, Expression, Pattern},
    Token,
};

mod else_block;

mod parse;
mod to_static;
mod to_tokens;

pub use else_block::ElseBlockExpression;

/// An conditional branch
#[derive(Debug, Clone)]
pub struct IfExpression<'a> {
    /// The token identifying this if statement
    pub r#if: Token![if],

    /// An optional variable to be defined in the block of the statement
    pub r#let: Option<(Token![let], Pattern<'a>, Token![=])>,

    /// The condition to evaluate
    pub condition: Box<Expression<'a>>,

    /// The block to execute if the condition is true
    pub block: BlockExpression<'a>,

    /// A block of code to execute if the condition is false
    pub r#else: Option<(Token![else], ElseBlockExpression<'a>)>,
}
