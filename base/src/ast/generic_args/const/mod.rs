use crate::{
    ast::{
        expressions::{BlockExpression, LiteralExpression},
        SimplePathSegment,
    },
    Token,
};

mod parse;
mod to_static;
mod to_tokens;

/// A constant generic argument
#[derive(Debug, Clone)]
pub enum GenericArgsConst<'a> {
    /// The argument is a block
    Block(BlockExpression<'a>),

    /// The argument is a literal
    Literal(LiteralExpression<'a>),

    /// The argument is a literal that starts with a dash
    DashLiteral(Token![-], LiteralExpression<'a>),

    /// The argument is a simple path segment
    SimplePathSegment(SimplePathSegment<'a>),
}
