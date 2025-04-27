use crate::{ast::ExpressionWithoutBlockKind, tokens::Literal, Parse, Parser, Result, Token};

impl<'a> Parse<'a> for ExpressionWithoutBlockKind<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if parser.peek::<Token![_]>() {
            return parser.parse().map(ExpressionWithoutBlockKind::Underscore);
        }

        if parser.peek::<&'a Literal>()
            || parser.peek::<Token![true]>()
            || parser.peek::<Token![false]>()
        {
            return parser.parse().map(ExpressionWithoutBlockKind::Literal);
        }

        if parser.peek::<Token![&]>() || parser.peek::<Token![&&]>() || parser.peek::<Token![*]>() {
            return parser.parse().map(ExpressionWithoutBlockKind::Operator);
        }

        if let Ok(macro_invocation) = parser.step_parse() {
            return Ok(ExpressionWithoutBlockKind::MacroInvocation(
                macro_invocation,
            ));
        }

        if let Ok(path) = parser.step_parse() {
            return Ok(ExpressionWithoutBlockKind::Path(path));
        }

        Err(parser.error("expected an expression without a block"))
    }
}
