use crate::{
    ast::expressions::BlockExpression, tokens::Group, Delimiter, Error, Parse, Parser, Result,
};

impl<'a> Parse<'a> for BlockExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let group: &'a Group = parser
            .parse()
            .map_err(|error| error.append("expected a block expression"))?;
        if group.delimiter != Delimiter::Brace {
            return Err(Error::new_at("expected a block expression", group.span));
        }

        let mut parser = group.parser();

        let attributes = parser.parse()?;
        let statements = parser.parse()?;
        let end = parser.parse()?;

        if !parser.empty() {
            return Err(parser.error("expected a block expression"));
        }

        Ok(BlockExpression {
            attributes,
            statements,
            end,
        })
    }
}
