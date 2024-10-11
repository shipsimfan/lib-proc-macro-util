use crate::{ast::QualifiedPathInType, Parse, Parser, Result};

impl<'a> Parse<'a> for QualifiedPathInType<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(QualifiedPathInType {
            r#type: parser.parse()?,
            segments: parser.parse()?,
        })
    }
}
