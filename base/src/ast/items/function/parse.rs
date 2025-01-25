use crate::{ast::items::Function, tokens::Group, Delimiter, Error, Parse, Parser, Result};

impl<'a> Parse<'a> for Function<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let qualifiers = parser.parse()?;
        let r#fn = parser.parse()?;
        let name = parser.parse()?;
        let generic_params = parser.parse()?;

        let group: &'a Group = parser.parse()?;
        if group.delimiter != Delimiter::Parenthesis {
            return Err(Error::new_at("expected function parameters", group.span));
        }

        let mut group_parser = group.parser();
        let parameters = group_parser.parse()?;
        if !group_parser.empty() {
            return Err(Error::new_at(
                "expected function parameters",
                group_parser.span(),
            ));
        }

        Ok(Function {
            qualifiers,
            r#fn,
            name,
            generic_params,
            parameters,
            return_type: parser.parse()?,
            where_clause: parser.parse()?,
            body: parser.parse()?,
        })
    }
}
