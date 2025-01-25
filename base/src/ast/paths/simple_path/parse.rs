use crate::{ast::SimplePath, Parse, Parser, Result, Token};

impl<'a> Parse<'a> for SimplePath<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let leading: Option<Token![::]> = parser.parse()?;
        let first = parser.parse().map_err(|error| {
            if leading.is_some() {
                error
            } else {
                parser.error("expected a simple path")
            }
        })?;
        let remaining = parser.parse()?;

        Ok(SimplePath {
            leading,
            first,
            remaining,
        })
    }
}
