use crate::{ast::expressions::OperatorExpression, Generator, ToTokens};

impl<'a> ToTokens for OperatorExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            OperatorExpression::Borrow(borrow) => borrow.to_tokens(generator),
            OperatorExpression::Dereference(dereference) => dereference.to_tokens(generator),
            OperatorExpression::Comparison(comparison) => comparison.to_tokens(generator),
            OperatorExpression::ErrorPropagation(error_propagation) => {
                error_propagation.to_tokens(generator)
            }
        }
    }
}
