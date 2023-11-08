use crate::parsing::Parse;

mod literal;

pub use literal::ExpressionLiteral;

#[non_exhaustive]
#[derive(Clone)]
pub enum Expression {
    Literal(ExpressionLiteral),
}

impl<'a> Parse<'a> for Expression {
    fn parse(parser: &mut crate::parsing::Parser<'a>) -> crate::Result<Self> {
        todo!()
    }
}
