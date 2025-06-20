use crate::{ast::items::StructBody, tokens::Group, Delimiter, Parse, Parser, Result};

impl<'a> Parse<'a> for StructBody<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(group) = parser.step_parse::<&'a Group>() {
            let mut group_parser = group.parser();

            match group.delimiter {
                Delimiter::Parenthesis => {
                    let fields = group_parser.parse()?;
                    if !group_parser.empty() {
                        return Err(group_parser.error("expected the end of structure fields"));
                    }

                    return Ok(StructBody::Tuple {
                        fields,
                        where_clause: parser.parse()?,
                        semi: parser.parse()?,
                    });
                }
                Delimiter::Brace => {
                    let fields = group_parser.parse()?;
                    if !group_parser.empty() {
                        return Err(group_parser.error("expected the end of structure fields"));
                    }

                    return Ok(StructBody::Normal {
                        where_clause: None,
                        fields,
                    });
                }
                _ => return Err(group.span.start().error("expected a `(` or `{`")),
            }
        }

        let where_clause = parser.parse()?;

        if let Ok(semi) = parser.step_parse() {
            return Ok(StructBody::Empty { where_clause, semi });
        }

        let group = parser.parse::<&'a Group>()?;
        if group.delimiter != Delimiter::Brace {
            return Err(group.span.start().error("expected a `{`"));
        }

        let mut group_parser = group.parser();
        let fields = group_parser.parse()?;
        if !group_parser.empty() {
            return Err(group_parser.error("expected the end of structure fields"));
        }

        Ok(StructBody::Normal {
            where_clause,
            fields,
        })
    }
}
