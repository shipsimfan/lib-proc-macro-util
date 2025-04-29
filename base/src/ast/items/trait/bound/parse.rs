use crate::{ast::TraitBound, tokens::Group, Delimiter, Parse, Parser, Result};

impl<'a> Parse<'a> for TraitBound<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let mut group = match parser.step(Parser::parse::<&'a Group>).ok() {
            Some(group) => {
                if group.delimiter != Delimiter::Parenthesis {
                    return Err(parser.error("expected a trait bound"));
                }

                Some(group.parser())
            }
            None => None,
        };

        let delimited = group.is_some();
        let parser = group.as_mut().unwrap_or(parser);

        Ok(TraitBound {
            delimited,
            question: parser.parse()?,
            for_lifetimes: parser.parse()?,
            path: parser.parse()?,
        })
    }
}
