use crate::{
    ast::{expressions::LoopLabel, ExpressionWithBlockKind},
    Parse, Parser, Result, Token,
};

impl<'a> Parse<'a> for ExpressionWithBlockKind<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if parser.peek::<Token![unsafe]>() {
            return parser.parse().map(ExpressionWithBlockKind::Unsafe);
        }

        if parser.peek::<Token![const]>() {
            return parser.parse().map(ExpressionWithBlockKind::Const);
        }

        if parser.peek::<Token![if]>() {
            return parser.parse().map(ExpressionWithBlockKind::If);
        }

        if let Ok(block) = parser.step_parse() {
            return Ok(ExpressionWithBlockKind::Block(block));
        }

        if parser.peek::<Token![for]>() || parser.peek::<LoopLabel<'a>>() {
            return parser.parse().map(ExpressionWithBlockKind::Loop);
        }

        Err(parser.error("expected an expression with a block"))
    }
}
