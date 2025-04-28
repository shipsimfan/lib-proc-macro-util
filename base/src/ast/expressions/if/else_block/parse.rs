use crate::{
    ast::expressions::{BlockExpression, ElseBlockExpression, IfExpression},
    Parse, Parser, Result, Token,
};

impl<'a> Parse<'a> for ElseBlockExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if parser.peek::<Token![if]>() {
            return IfExpression::parse(parser).map(|r#if| ElseBlockExpression::If(Box::new(r#if)));
        }

        BlockExpression::parse(parser).map(ElseBlockExpression::Block)
    }
}
