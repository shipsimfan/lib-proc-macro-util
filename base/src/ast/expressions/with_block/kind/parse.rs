use crate::{ast::ExpressionWithBlockKind, Parse, Parser, Result, Token};

impl<'a> Parse<'a> for ExpressionWithBlockKind<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if parser.peek::<Token![unsafe]>() {
            return parser.parse().map(ExpressionWithBlockKind::Unsafe);
        }

        if let Ok(block) = parser.step_parse() {
            return Ok(ExpressionWithBlockKind::Block(block));
        }

        Err(parser.error("expected an expression with a block"))
    }
}
