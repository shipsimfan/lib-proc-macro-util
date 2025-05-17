use crate::{ast::expressions::LazyBooleanOperator, Generator, ToTokens};

impl ToTokens for LazyBooleanOperator {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            LazyBooleanOperator::Or(or) => or.to_tokens(generator),
            LazyBooleanOperator::And(and) => and.to_tokens(generator),
        }
    }
}
