use crate::{ast::Visibility, tokens::Group, Delimiter, Parse, Parser, Result};

impl<'a> Parse<'a> for Visibility<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let r#pub = parser.parse()?;
        let scope = match parser.step::<&'a Group, _>(Parser::parse) {
            Ok(group) => {
                if group.delimiter != Delimiter::Parenthesis {
                    return Err(group.span.error("expected `(`"));
                }

                group.parser().parse()?
            }
            Err(_) => None,
        };

        Ok(Visibility { r#pub, scope })
    }
}
