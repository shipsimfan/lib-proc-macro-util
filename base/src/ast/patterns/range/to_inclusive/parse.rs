use crate::{ast::patterns::RangeToInclusivePattern, Parse, Parser, Result};

impl<'a> Parse<'a> for RangeToInclusivePattern<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(RangeToInclusivePattern {
            dots: parser.parse()?,
            upper: parser.parse()?,
        })
    }
}
