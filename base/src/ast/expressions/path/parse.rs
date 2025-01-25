use crate::{ast::expressions::PathExpression, Parse, Parser, Result};

impl<'a> Parse<'a> for PathExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(path) = parser.step_parse() {
            return Ok(PathExpression::Normal(path));
        }

        if let Ok(qualified_path) = parser.step_parse() {
            return Ok(PathExpression::QualifiedPathInExpression(qualified_path));
        }

        Err(parser.error("expected a path"))
    }
}
