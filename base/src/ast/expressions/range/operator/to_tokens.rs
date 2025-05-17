use crate::{ast::expressions::RangeOperator, Generator, ToTokens};

impl ToTokens for RangeOperator {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            RangeOperator::Exclusive(exclusive) => exclusive.to_tokens(generator),
            RangeOperator::Inclusive(inclusive) => inclusive.to_tokens(generator),
        }
    }
}
