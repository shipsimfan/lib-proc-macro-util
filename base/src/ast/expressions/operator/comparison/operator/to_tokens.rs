use crate::{ast::expressions::ComparisonOperator, Generator, ToTokens};

impl ToTokens for ComparisonOperator {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            ComparisonOperator::Equals(equals) => equals.to_tokens(generator),
            ComparisonOperator::NotEquals(not_equals) => not_equals.to_tokens(generator),
            ComparisonOperator::GreaterThan(greater_than) => greater_than.to_tokens(generator),
            ComparisonOperator::LessThan(less_than) => less_than.to_tokens(generator),
            ComparisonOperator::GreaterThanOrEqual(greater_than_or_equal) => {
                greater_than_or_equal.to_tokens(generator)
            }
            ComparisonOperator::LessThanOrEqual(less_than_or_equal) => {
                less_than_or_equal.to_tokens(generator)
            }
        }
    }
}
