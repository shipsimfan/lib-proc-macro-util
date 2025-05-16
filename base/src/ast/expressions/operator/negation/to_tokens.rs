use crate::{ast::expressions::NegationExpression, Generator, ToTokens};

impl<'a> ToTokens for NegationExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.operator.to_tokens(generator);
        self.expression.to_tokens(generator);
    }
}
