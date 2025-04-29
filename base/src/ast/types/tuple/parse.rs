use crate::{ast::types::TupleType, tokens::Group, Delimiter, Parse, Parser};

impl<'a> Parse<'a> for TupleType<'a> {
    fn parse(parser: &mut Parser<'a>) -> crate::Result<Self> {
        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Parenthesis {
            return Err(parser.error("expected a tuple"));
        }

        let mut parser = group.parser();

        Ok(TupleType {
            types: parser.parse()?,
            last: parser.parse()?,
        })
    }
}
