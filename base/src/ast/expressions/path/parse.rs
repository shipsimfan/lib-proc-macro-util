use crate::{ast::expressions::PathExpression, supported_languages::*, Parse, Parser, Result};
use i18n_translation::m;

i18n_translation::message_key!(ExpectedPath [
    EN => { "expected a path" },
    FR => { "un chemin était attendu" },
    ZH => { "预期的路径" },
]);

impl<'a> Parse<'a> for PathExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(path) = parser.step_parse() {
            return Ok(PathExpression::Normal(path));
        }

        if let Ok(qualified_path) = parser.step_parse() {
            return Ok(PathExpression::QualifiedPathInExpression(qualified_path));
        }

        Err(parser.error(m!(ExpectedPath)))
    }
}
