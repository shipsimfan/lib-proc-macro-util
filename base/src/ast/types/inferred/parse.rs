use crate::{ast::types::InferredType, Parse, Parser, Result};

impl<'a> Parse<'a> for InferredType {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(InferredType(parser.parse()?))
    }
}
