use crate::{ast::expressions::LoopExpressionKind, Error, Parse, Parser, Result, Token};

impl<'a> Parse<'a> for LoopExpressionKind<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if parser.peek::<Token![for]>() {
            return parser.parse().map(LoopExpressionKind::Iterator);
        }

        if parser.peek::<Token![loop]>() {
            return parser.parse().map(LoopExpressionKind::Infinite);
        }

        Err(Error::new_at("expected a loop expression", parser.span()))
    }
}
