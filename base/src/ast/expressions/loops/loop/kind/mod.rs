use crate::ast::expressions::{
    BlockExpression, InfiniteLoopExpression, IteratorLoopExpression, PredicateLoopExpression,
};

mod parse;
mod to_static;
mod to_tokens;

/// The kind of loop a loop expression is
#[derive(Debug, Clone)]
pub enum LoopExpressionKind<'a> {
    /// The expression loops over the results of an iterator
    Iterator(IteratorLoopExpression<'a>),

    /// An expression that loops continously
    Infinite(InfiniteLoopExpression<'a>),

    /// An expression that evaluates a condition each loop
    Predicate(PredicateLoopExpression<'a>),

    /// A block with a label
    Block(BlockExpression<'a>),
}
