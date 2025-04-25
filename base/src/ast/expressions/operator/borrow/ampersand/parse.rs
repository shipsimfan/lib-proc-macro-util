use crate::{ast::expressions::BorrowAmpersand, Parse, Parser, Result};

impl<'a> Parse<'a> for BorrowAmpersand {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(ampersand) = parser.step_parse() {
            return Ok(BorrowAmpersand::Double(ampersand));
        }

        parser.parse().map(BorrowAmpersand::Single)
    }
}
