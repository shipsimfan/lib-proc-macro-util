use crate::{
    ast::{expressions::CallParams, Expression},
    tokens::Group,
    Delimiter, Error, Parse, Parser, Result,
};

impl<'a> Parse<'a> for CallParams<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Parenthesis {
            return Err(Error::new_at(
                "function call parameters must be surrounded by parentheses",
                group.span,
            ));
        }

        let mut parser = group.parser();

        let first: Option<Box<Expression>> = parser.parse()?;
        let (remaining, last) = if first.is_some() {
            (parser.parse()?, parser.parse()?)
        } else {
            (Vec::new(), None)
        };

        if !parser.empty() {
            return Err(Error::new_at("unexpected token", parser.span()));
        }

        Ok(CallParams {
            first,
            remaining,
            last,
        })
    }
}
