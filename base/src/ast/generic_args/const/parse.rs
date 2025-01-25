use crate::{ast::GenericArgsConst, Parse, Parser, Result};

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

        Err(parser.error("expected a constant generic argument"))
    }
}
