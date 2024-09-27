use crate::{
    ast::{
        expressions::{BlockExpression, LiteralExpression},
        SimplePathSegment,
    },
    Token,
};

pub enum GenericArgsConst<'a> {
    Block(BlockExpression<'a>),
    Literal(LiteralExpression<'a>),
    DashLiteral(Token![-], LiteralExpression<'a>),
    SimplePathSegment(SimplePathSegment<'a>),
}
