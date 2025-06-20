use crate::{ast::expressions::ComparisonOperator, Parse, Parser, Result};

impl<'a> Parse<'a> for ComparisonOperator {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        if let Ok(equals) = parser.step_parse() {
            return Ok(ComparisonOperator::Equals(equals));
        }

        if let Ok(not_equals) = parser.step_parse() {
            return Ok(ComparisonOperator::NotEquals(not_equals));
        }

        if let Ok(greater_than) = parser.step_parse() {
            return Ok(ComparisonOperator::GreaterThan(greater_than));
        }

        if let Ok(less_than) = parser.step_parse() {
            return Ok(ComparisonOperator::LessThan(less_than));
        }

        if let Ok(greater_than_or_equal) = parser.step_parse() {
            return Ok(ComparisonOperator::GreaterThanOrEqual(
                greater_than_or_equal,
            ));
        }

        if let Ok(less_than_or_equal) = parser.step_parse() {
            return Ok(ComparisonOperator::LessThanOrEqual(less_than_or_equal));
        }

        Err(parser.error("unexpected token"))
    }
}
