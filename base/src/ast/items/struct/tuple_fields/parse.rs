use crate::{ast::items::TupleFields, Parse, Parser, Result};

impl<'a> Parse<'a> for TupleFields<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(TupleFields {
            first: parser.parse()?,
            remaining: parser.parse()?,
            last: parser.parse()?,
        })
    }
}
