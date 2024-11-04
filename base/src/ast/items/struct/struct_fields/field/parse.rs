use std::borrow::Cow;

use crate::{ast::items::StructField, Parse, Parser, Result};

impl<'a> Parse<'a> for StructField<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(StructField {
            attributes: parser.parse()?,
            visibility: parser.parse()?,
            name: Cow::Borrowed(parser.parse()?),
            colon: parser.parse()?,
            r#type: parser.parse()?,
        })
    }
}
