use crate::{ast::patterns::TupleStructItems, Parse, Parser, Result};

impl<'a> Parse<'a> for TupleStructItems<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(TupleStructItems {
            first: parser.parse()?,
            remaining: parser.parse()?,
            last: parser.parse()?,
        })
    }
}
