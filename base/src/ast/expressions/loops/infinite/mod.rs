use crate::{ast::expressions::BlockExpression, Token};

mod parse;
mod to_static;
mod to_tokens;

/// A loop expression repeats execution of its body continuously
#[derive(Debug, Clone)]
pub struct InfiniteLoopExpression<'a> {
    /// The `loop` token indicating this type of loop
    pub r#loop: Token![loop],

    /// The body of the loop to execute each iteration
    pub block: BlockExpression<'a>,
}
