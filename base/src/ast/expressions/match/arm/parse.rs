use crate::{ast::expressions::MatchArm, Parse, Parser, Result, Token};

impl<'a> Parse<'a> for MatchArm<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let attributes = parser.parse()?;
        let pattern = parser.parse()?;

        let guard = if parser.peek::<Token![if]>() {
            Some(parser.parse()?)
        } else {
            None
        };

        Ok(MatchArm {
            attributes,
            pattern,
            guard,
        })
    }
}
