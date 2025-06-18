use crate::{ast::SimplePath, Parse, Parser, Result};

impl<'a> Parse<'a> for SimplePath<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let leading = parser.parse()?;
        let first = parser.parse()?;

        let mut remaining = Vec::new();
        while let Ok(path_sep) = parser.step_parse() {
            remaining.push((path_sep, parser.parse()?));
        }

        Ok(SimplePath {
            leading,
            first,
            remaining,
        })
    }
}
