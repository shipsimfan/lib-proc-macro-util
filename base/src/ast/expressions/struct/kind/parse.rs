use crate::{
    ast::expressions::StructExprKind, tokens::Group, Delimiter, Error, Parse, Parser, Result,
};

impl<'a> Parse<'a> for StructExprKind<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(group) = parser.step_parse::<&'a Group>() {
            let mut parser = group.parser();
            let kind = match group.delimiter {
                Delimiter::Brace => StructExprKind::Struct(parser.parse()?),
                Delimiter::Parenthesis => StructExprKind::Tuple(parser.parse()?),
                _ => return Err(Error::new_at("unexpected token", group.span)),
            };

            if !parser.empty() {
                return Err(parser.error("unexpected token"));
            }

            return Ok(kind);
        }

        Ok(StructExprKind::Unit)
    }
}
