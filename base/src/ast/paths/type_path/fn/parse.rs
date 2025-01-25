use crate::{ast::TypePathFn, tokens::Group, Delimiter, Parse, Parser, Result};

impl<'a> Parse<'a> for TypePathFn<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Parenthesis {
            return Err(parser.error("expected function inputs"));
        }

        let mut group_parser = group.parser();
        let inputs = group_parser.parse()?;
        if !group_parser.empty() {
            return Err(parser.error("expected the end of function inputs"));
        }

        let r#return = parser.parse()?;

        Ok(TypePathFn { inputs, r#return })
    }
}
