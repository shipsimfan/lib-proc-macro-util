use crate::{ast::expressions::ErrorPropagationExpression, Generator, ToTokens};

impl<'a> ToTokens for ErrorPropagationExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.expression.to_tokens(generator);
        self.question.to_tokens(generator);
    }
}
