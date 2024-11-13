use crate::{ast::GenericArgsConst, supported_languages::*, Parse, Parser, Result};
use i18n_translation::m;

i18n_translation::message_key!(ExpectedConstantGenericArg [
    EN => { "expected a constant generic argument" },
    FR => { "un argument générique constant était attendu" },
    ZH => { "预期的常量泛型参数" },
]);

impl<'a> Parse<'a> for GenericArgsConst<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(block) = parser.step_parse() {
            return Ok(GenericArgsConst::Block(block));
        }

        if let Ok(literal) = parser.step_parse() {
            return Ok(GenericArgsConst::Literal(literal));
        }

        if let Ok(dash) = parser.step_parse() {
            return Ok(GenericArgsConst::DashLiteral(dash, parser.parse()?));
        }

        if let Ok(simple_path_segment) = parser.step_parse() {
            return Ok(GenericArgsConst::SimplePathSegment(simple_path_segment));
        }

        Err(parser.error(m!(ExpectedConstantGenericArg)))
    }
}
