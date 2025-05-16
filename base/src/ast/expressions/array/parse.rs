use crate::{
    ast::expressions::ArrayExpression, tokens::Group, Delimiter, Error, Parse, Parser, Result,
};

impl<'a> Parse<'a> for ArrayExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let group: &'a Group = parser
            .parse()
            .map_err(|error| error.append("expected an array expression"))?;
        if group.delimiter != Delimiter::Parenthesis {
            return Err(Error::new_at("expected an array expression", group.span));
        }

        let mut parser = group.parser();

        let elements = parser.parse()?;

        if !parser.empty() {
            return Err(parser.error("unexpected token"));
        }

        Ok(ArrayExpression { elements })
    }
}
