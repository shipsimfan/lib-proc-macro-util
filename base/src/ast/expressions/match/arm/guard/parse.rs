use crate::{ast::expressions::MatchArmGuard, Parse, Parser, Result};

impl<'a> Parse<'a> for MatchArmGuard<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(MatchArmGuard {
            r#if: parser.parse()?,
            expression: parser.parse()?,
        })
    }
}
