use crate::{
    ast::{Expression, TypeNoBounds},
    Token,
};

mod parse;
mod to_static;
mod to_tokens;

/// A type cast expression is denoted with the binary operator `as`
#[derive(Debug, Clone)]
pub struct TypeCastExpression<'a> {
    /// The expression to be evaluated for errors
    pub expression: Box<Expression<'a>>,

    /// The `as` keyword identifying this expression
    pub r#as: Token![as],

    /// The type to be cast to
    pub r#type: Box<TypeNoBounds<'a>>,
}
