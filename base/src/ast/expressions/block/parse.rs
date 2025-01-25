use crate::{ast::expressions::BlockExpression, tokens::Group, Delimiter, Parse, Parser, Result};

impl<'a> Parse<'a> for BlockExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let group: &'a Group = parser
            .parse()
            .map_err(|error| error.append("expected a block expression"))?;
        if group.delimiter != Delimiter::Brace {
            return Err(parser.error("expected a block expression"));
        }

        let mut group = group.parser();

        let attributes = group.parse()?;
        let statements = group.parse()?;
        let end = group.parse()?;

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
