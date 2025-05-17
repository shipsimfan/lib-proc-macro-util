use crate::{ast::expressions::RangeOperator, Parse, Parser, Result};

impl<'a> Parse<'a> for RangeOperator {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(inclusive) = parser.step_parse() {
            return Ok(RangeOperator::Inclusive(inclusive));
        }

        parser.parse().map(RangeOperator::Exclusive)
    }
}
