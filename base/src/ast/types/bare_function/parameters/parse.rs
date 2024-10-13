use crate::{ast::types::MaybeNamedFunctionParameters, Parse, Parser, Result};

impl<'a> Parse<'a> for MaybeNamedFunctionParameters<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(MaybeNamedFunctionParameters {
            first: parser.parse()?,
            remaining: parser.parse()?,
            variadic: parser.parse()?,
            ending: parser.parse()?,
        })
    }
}
