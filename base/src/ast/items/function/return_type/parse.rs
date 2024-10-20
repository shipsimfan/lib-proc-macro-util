use crate::{ast::items::FunctionReturnType, Parse, Parser, Result};

impl<'a> Parse<'a> for FunctionReturnType<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(FunctionReturnType {
            arrow: parser.parse()?,
            r#type: parser.parse()?,
        })
    }
}
