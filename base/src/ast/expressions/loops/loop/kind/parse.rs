use crate::{
    ast::expressions::{IteratorLoopExpression, LoopExpressionKind},
    Error, Parse, Parser, Result, Token,
};

impl<'a> Parse<'a> for LoopExpressionKind<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if parser.peek::<Token![for]>() {
            return IteratorLoopExpression::parse(parser).map(LoopExpressionKind::Iterator);
        }

        Err(Error::new_at("expected a loop expression", parser.span()))
    }
}
