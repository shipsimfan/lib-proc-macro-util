use crate::{
    ast::{expressions::MatchExpression, Expression},
    tokens::Group,
    Delimiter, Parse, Parser, Result,
};

impl<'a> Parse<'a> for MatchExpression<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let r#match = parser.parse()?;
        let scrutinee = Box::new(Expression::parse_without_struct(parser)?);

        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Brace {
            return Err(group.span.start().error("expected `[`"));
        }

        let mut parser = group.parser();
        let attributes = parser.parse()?;
        let mut arms = Vec::new();
        while !parser.empty() {
            let arm = parser.parse()?;
            let arrow = parser.parse()?;
            let expression = parser.parse()?;
            let comma = parser.parse()?;

            let r#break = match (&expression, comma) {
                (Expression::WithoutBlock(_), None) => true,
                _ => false,
            };

            arms.push((arm, arrow, expression, comma));

            if r#break {
                break;
            }
        }

        if !parser.empty() {
            return Err(parser.error("expected `]`"));
        }

        Ok(MatchExpression {
            r#match,
            scrutinee,
            attributes,
            arms,
        })
    }
}
