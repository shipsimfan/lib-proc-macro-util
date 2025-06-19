/*use crate::ast::expressions::{
    BlockExpression, ConstBlockExpression, IfExpression, LoopExpression, MatchExpression,
                    UnsafeBlockExpression,
};*/

use std::marker::PhantomData;

//mod from;
mod new;
mod parse;
mod to_static;
mod to_tokens;

/// A specific type of expression that has a block
#[derive(Debug, Clone)]
pub enum ExpressionWithBlockKind<'a> {
    /// TODO: Remove
    Phantom(PhantomData<&'a ()>),
    /*
    /// An expression made up of only a block
    Block(BlockExpression<'a>),

    /// A block of unsafe code
    Unsafe(UnsafeBlockExpression<'a>),

    /// A block of constant code
    Const(ConstBlockExpression<'a>),

    /// A block of code that loops
    Loop(LoopExpression<'a>),

    /// A conditional expression
    If(IfExpression<'a>),

    /// An expression which runs code conditionally on matching a pattern
    Match(MatchExpression<'a>),
    */
}
