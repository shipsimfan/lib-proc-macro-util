use crate::{
    ast::expressions::GroupedExpression, tokens::Group, Delimiter, Error, Parse, Parser, Result,
};

impl<'a> Parse<'a> for GroupedExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let group: &'a Group = parser
            .parse()
            .map_err(|error| error.append("expected a grouped expression"))?;
        if group.delimiter != Delimiter::Parenthesis {
            return Err(Error::new_at("expected a grouped expression", group.span));
        }

        let mut parser = group.parser();

        let expression = parser.parse()?;

        if !parser.empty() {
            return Err(parser.error("unexpected error"));
        }

        Ok(GroupedExpression { expression })
    }
}
