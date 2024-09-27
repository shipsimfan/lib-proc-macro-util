use crate::{
    ast::{
        expressions::{BlockExpression, LiteralExpression},
        SimplePathSegment,
    },
    Token,
};

mod parse;
mod to_tokens;

#[derive(Debug, Clone)]
pub enum GenericArgsConst<'a> {
    Block(BlockExpression<'a>),
    Literal(LiteralExpression<'a>),
    DashLiteral(Token![-], LiteralExpression<'a>),
    SimplePathSegment(SimplePathSegment<'a>),
}
