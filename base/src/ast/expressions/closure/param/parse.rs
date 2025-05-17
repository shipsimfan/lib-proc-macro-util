use crate::{ast::expressions::ClosureParam, Parse, Parser, Result};

impl<'a> Parse<'a> for ClosureParam<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(ClosureParam {
            attributes: parser.parse()?,
            pattern: parser.parse()?,
            r#type: parser.parse()?,
        })
    }
}
