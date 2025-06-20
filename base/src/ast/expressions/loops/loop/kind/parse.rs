use crate::{ast::expressions::LoopExpressionKind, tokens::Group, Parse, Parser, Result, Token};

impl<'a> Parse<'a> for LoopExpressionKind<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if parser.peek::<Token![for]>() {
            return parser.parse().map(LoopExpressionKind::Iterator);
        }

        if parser.peek::<Token![loop]>() {
            return parser.parse().map(LoopExpressionKind::Infinite);
        }

        if parser.peek::<Token![while]>() {
            return parser.parse().map(LoopExpressionKind::Predicate);
        }

        if parser.peek::<&'a Group>() {
            return parser.parse().map(LoopExpressionKind::Block);
        }

        Err(parser.error("expected a loop expression"))
    }
}
