mod borrow;
mod dereference;

mod from;
mod new;
mod parse;
mod to_static;
mod to_tokens;

pub use borrow::*;
pub use dereference::DereferenceExpression;

/// An expression that contains an operator
#[derive(Debug, Clone)]
pub enum OperatorExpression<'a> {
    /// A borrow of another expression
    Borrow(BorrowExpression<'a>),

    /// A dereference of another expression
    Dereference(DereferenceExpression<'a>),
}
