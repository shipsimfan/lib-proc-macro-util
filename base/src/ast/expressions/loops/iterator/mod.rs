use crate::{
    ast::{expressions::BlockExpression, Expression, Pattern},
    Token,
};

mod parse;
mod to_static;
mod to_tokens;

/// An expression that loops over values extracted from an iterator
#[derive(Debug, Clone)]
pub struct IteratorLoopExpression<'a> {
    /// The `for` token indicating this type of loop
    pub r#for: Token![for],

    /// The pattern holding the results of the iterator
    pub pattern: Pattern<'a>,

    /// The `in` token separating the pattern and iterator
    pub r#in: Token![in],

    /// The iterator to extract values from
    pub iterator: Box<Expression<'a>>,

    /// The body of the loop to execute for each item
    pub block: BlockExpression<'a>,
}
