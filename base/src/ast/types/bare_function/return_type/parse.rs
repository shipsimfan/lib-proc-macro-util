use crate::{ast::types::BareFunctionReturnType, Parse, Parser, Result};

impl<'a> Parse<'a> for BareFunctionReturnType<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(BareFunctionReturnType {
            right_arrow: parser.parse()?,
            r#type: parser.parse()?,
        })
    }
}
