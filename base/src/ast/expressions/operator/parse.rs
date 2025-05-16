use crate::{ast::expressions::OperatorExpression, Parse, Parser, Result, Token};

impl<'a> Parse<'a> for OperatorExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if parser.peek::<Token![*]>() {
            return parser.parse().map(OperatorExpression::Dereference);
        }

        if parser.peek::<Token![&]>() || parser.peek::<Token![&&]>() {
            return parser.parse().map(OperatorExpression::Borrow);
        }

        if parser.peek::<Token![-]>() || parser.peek::<Token![!]>() {
            return parser.parse().map(OperatorExpression::Negation);
        }

        Err(parser.error("expected an opeator expression"))
    }
}
