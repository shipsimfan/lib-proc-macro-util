use crate::{
    parsing::{Parse, Parser},
    tokens::{Ident, Literal},
    Result,
};

mod literal;

pub use literal::ExpressionLiteral;

#[non_exhaustive]
#[derive(Clone)]
pub enum Expression {
    Literal(ExpressionLiteral),
}

impl Expression {
    pub(crate) fn constant(parser: &mut Parser) -> Result<Expression> {
        if parser.peek::<Literal>() {
            return Ok(Expression::Literal(parser.parse()?));
        }

        if parser.peek::<Ident>() {
            todo!()
        }

        todo!()
    }
}

impl<'a> Parse<'a> for Expression {
    fn parse(parser: &mut crate::parsing::Parser<'a>) -> crate::Result<Self> {
        todo!()
    }
}
