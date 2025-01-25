use crate::{ast::ConstParamValue, Parse, Parser, Result};

impl<'a> Parse<'a> for ConstParamValue<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(block) = parser.step_parse() {
            return Ok(ConstParamValue::Block(block));
        }

        if let Ok(identifier) = parser.step_parse() {
            return Ok(ConstParamValue::Identifier(identifier));
        }

        Ok(ConstParamValue::Literal(
            parser.parse()?,
            parser
                .parse()
                .map_err(|error| error.append_at("expected a constant value", parser.span()))?,
        ))
    }
}
