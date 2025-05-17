use crate::{
    ast::{expressions::RangeExpression, ExpressionWithoutBlockKind},
    tokens::{Group, Literal},
    Parse, Parser, Result, Token,
};

impl<'a> Parse<'a> for ExpressionWithoutBlockKind<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if parser.peek::<Token![_]>() {
            return parser.parse().map(ExpressionWithoutBlockKind::Underscore);
        }

        if parser.peek::<Token![continue]>() {
            return parser.parse().map(ExpressionWithoutBlockKind::Continue);
        }

        if parser.peek::<Token![break]>() {
            return parser.parse().map(ExpressionWithoutBlockKind::Break);
        }

        if parser.peek::<Token![return]>() {
            return parser.parse().map(ExpressionWithoutBlockKind::Return);
        }

        if let Ok(closure) = parser.step_parse() {
            return Ok(ExpressionWithoutBlockKind::Closure(closure));
        }

        if parser.peek::<Token![async]>() {
            return parser.parse().map(ExpressionWithoutBlockKind::AsyncBlock);
        }

        if let Ok(operator) = parser.step_parse() {
            return Ok(ExpressionWithoutBlockKind::Range(RangeExpression {
                lower: None,
                operator,
                upper: parser.parse()?,
            }));
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

        if parser.peek::<&'a Group>() {
            if let Ok(grouped) = parser.step_parse() {
                return Ok(ExpressionWithoutBlockKind::Grouped(grouped));
            }

            if let Ok(tuple) = parser.step_parse() {
                return Ok(ExpressionWithoutBlockKind::Tuple(tuple));
            }

            if let Ok(array) = parser.step_parse() {
                return Ok(ExpressionWithoutBlockKind::Array(array));
            }
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
