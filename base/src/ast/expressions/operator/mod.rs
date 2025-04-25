mod borrow;

mod from;
mod new;
mod parse;
mod to_static;
mod to_tokens;

pub use borrow::*;

/// An expression that contains an operator
#[derive(Debug, Clone)]
pub enum OperatorExpression<'a> {
    /// A borrow of another expression
    Borrow(BorrowExpression<'a>),
}
