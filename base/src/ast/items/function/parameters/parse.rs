use crate::{ast::items::FunctionParameters, Parse, Parser, Result};

impl<'a> Parse<'a> for FunctionParameters<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok((_self, first)) = parser.step_parse() {
            return Ok(FunctionParameters::Normal {
                _self,
                first,
                remaining: parser.parse()?,
                ending: parser.parse()?,
            });
        }

        Ok(FunctionParameters::OnlySelf {
            _self: parser.parse()?,
            comma: parser.parse()?,
        })
    }
}
