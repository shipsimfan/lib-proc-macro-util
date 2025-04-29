use crate::{ast::types::BareFunctionType, tokens::Group, Delimiter, Parse, Parser, Result};

impl<'a> Parse<'a> for BareFunctionType<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let for_lifetimes = parser.parse()?;
        let qualifiers = parser.parse()?;
        let r#fn = parser.parse()?;

        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Parenthesis {
            return Err(parser.error("expected function parameters"));
        }

        let parameters = group.parser().parse()?;

        Ok(BareFunctionType {
            for_lifetimes,
            qualifiers,
            r#fn,
            parameters,
            return_type: parser.parse()?,
        })
    }
}
