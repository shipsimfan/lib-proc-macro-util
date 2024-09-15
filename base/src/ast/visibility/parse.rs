use crate::{ast::Visibility, tokens::Group, Delimiter, Error, Parse, Parser, Result};

impl<'a> Parse<'a> for Visibility<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let r#pub = parser.parse()?;
        let scope = match parser.step::<&'a Group, _>(Parser::parse) {
            Ok(group) => {
                if group.delimiter != Delimiter::Parenthesis {
                    return Err(Error::new_at(
                        "expected a group delimited by parentheses",
                        group.span,
                    ));
                }

                group.parser().parse()?
            }
            Err(_) => None,
        };

        Ok(Visibility { r#pub, scope })
    }
}
