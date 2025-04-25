use crate::{ast::ExpressionKind, tokens::Literal, Parse, Parser, Result, Token};

impl<'a> Parse<'a> for ExpressionKind<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if parser.peek::<Token![unsafe]>() {
            return parser.parse().map(ExpressionKind::Unsafe);
        }

        if parser.peek::<&'a Literal>()
            || parser.peek::<Token![true]>()
            || parser.peek::<Token![false]>()
        {
            return parser.parse().map(ExpressionKind::Literal);
        }

        if parser.peek::<Token![&]>() || parser.peek::<Token![&&]>() || parser.peek::<Token![*]>() {
            return parser.parse().map(ExpressionKind::Operator);
        }

        if let Ok(macro_invocation) = parser.step_parse() {
            return Ok(ExpressionKind::MacroInvocation(macro_invocation));
        }

        if let Ok(block) = parser.step_parse() {
            return Ok(ExpressionKind::Block(block));
        }

        if let Ok(path) = parser.step_parse() {
            return Ok(ExpressionKind::Path(path));
        }

        Err(parser.error("expected an expression"))
    }
}
