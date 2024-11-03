use crate::{ast::items::TupleField, Parse, Parser, Result};

impl<'a> Parse<'a> for TupleField<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(TupleField {
            attributes: parser.parse()?,
            visibility: parser.parse()?,
            r#type: parser.parse()?,
        })
    }
}
