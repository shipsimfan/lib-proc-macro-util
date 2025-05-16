mod borrow;
mod comparison;
mod dereference;
mod error_propagation;
mod type_cast;

mod from;
mod new;
mod parse;
mod to_static;
mod to_tokens;

pub use borrow::*;
pub use comparison::{ComparisonExpression, ComparisonOperator};
pub use dereference::DereferenceExpression;
pub use error_propagation::ErrorPropagationExpression;
pub use type_cast::TypeCastExpression;

/// An expression that contains an operator
#[derive(Debug, Clone)]
pub enum OperatorExpression<'a> {
    /// A borrow of another expression
    Borrow(BorrowExpression<'a>),

    /// A dereference of another expression
    Dereference(DereferenceExpression<'a>),

    /// A comparison between two expressions
    Comparison(ComparisonExpression<'a>),

    /// Unwraps valid values or propagates errors
    ErrorPropagation(ErrorPropagationExpression<'a>),

    /// Converts an expression to a type
    TypeCast(TypeCastExpression<'a>),
}
