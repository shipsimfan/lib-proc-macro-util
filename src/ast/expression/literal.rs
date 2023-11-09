use crate::{
    ast::Attribute,
    parsing::{Parse, Parser},
    tokens::Literal,
    Result,
};

#[derive(Clone)]
pub struct ExpressionLiteral {
    pub attributes: Vec<Attribute>,
    pub literal: Literal,
}

impl<'a> Parse<'a> for ExpressionLiteral {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        todo!()
    }
}
