use crate::{ast::expressions::StructExprField, Parse, Parser, Result};

impl<'a> Parse<'a> for StructExprField<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(StructExprField {
            attributes: parser.parse()?,
            name: parser.parse()?,
            expression: match parser.step_parse() {
                Ok(colon) => Some((colon, parser.parse()?)),
                Err(_) => None,
            },
        })
    }
}
