use crate::{ast::expressions::NegationOperator, Generator, ToTokens};

impl ToTokens for NegationOperator {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            NegationOperator::Numeric(dash) => dash.to_tokens(generator),
            NegationOperator::Boolean(exclamation) => exclamation.to_tokens(generator),
        }
    }
}
