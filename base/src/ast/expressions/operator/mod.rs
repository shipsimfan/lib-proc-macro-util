mod arithmetic_or;
mod borrow;
mod comparison;
mod dereference;
mod error_propagation;
mod negation;
mod type_cast;

mod from;
mod new;
mod parse;
mod to_static;
mod to_tokens;

pub use arithmetic_or::{ArithmeticOrLogicalExpression, ArithmeticOrLogicalOperator};
pub use borrow::*;
pub use comparison::{ComparisonExpression, ComparisonOperator};
pub use dereference::DereferenceExpression;
pub use error_propagation::ErrorPropagationExpression;
pub use negation::{NegationExpression, NegationOperator};
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

    /// Negates another expression
    Negation(NegationExpression<'a>),

    /// A arithmetic or logical operation between two expressions
    ArithmeticOrLogical(ArithmeticOrLogicalExpression<'a>),
}
