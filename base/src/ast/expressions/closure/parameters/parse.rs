use crate::{ast::expressions::ClosureParameters, Parse, Parser, Result};

impl<'a> Parse<'a> for ClosureParameters<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(ClosureParameters {
            first: parser.parse()?,
            remaining: parser.parse()?,
            last: parser.parse()?,
        })
    }
}
