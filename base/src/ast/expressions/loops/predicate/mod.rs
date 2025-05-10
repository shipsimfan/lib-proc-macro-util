use crate::{
    ast::{expressions::BlockExpression, Expression, Pattern},
    Token,
};

mod parse;
mod to_static;
mod to_tokens;

/// A loop that evaluates a boolean expression each iteration
#[derive(Debug, Clone)]
pub struct PredicateLoopExpression<'a> {
    /// The `while` token indicating this type of loop
    pub r#while: Token![while],

    /// An optional variable to be defined in the block of the statement
    pub r#let: Option<(Token![let], Pattern<'a>, Token![=])>,

    /// The condition to evaluate
    pub condition: Box<Expression<'a>>,

    /// The body of the loop to execute each iteration
    pub block: BlockExpression<'a>,
}
