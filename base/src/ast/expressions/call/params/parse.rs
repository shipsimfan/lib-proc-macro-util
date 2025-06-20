use crate::{
    ast::{expressions::CallParams, Expression},
    tokens::Group,
    Delimiter, Parse, Parser, Result,
};

impl<'a> Parse<'a> for CallParams<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let group: &'a Group = match parser.step_parse() {
            Ok(group) => group,
            Err(_) => return Err(parser.error("expected `(`")),
        };
        if group.delimiter != Delimiter::Parenthesis {
            return Err(group.span.start().error("expected `(`"));
        }

        let mut parser = group.parser();

        let first: Option<Box<Expression>> = parser.parse()?;
        let (remaining, last) = if first.is_some() {
            (parser.parse()?, parser.parse()?)
        } else {
            (Vec::new(), None)
        };

        if !parser.empty() {
            return Err(parser.error("expected `)`"));
        }

        Ok(CallParams {
            first,
            remaining,
            last,
        })
    }
}
