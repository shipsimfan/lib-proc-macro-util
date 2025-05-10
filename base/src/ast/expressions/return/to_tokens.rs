use crate::{ast::expressions::ReturnExpression, Generator, ToTokens};

impl<'a> ToTokens for ReturnExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#return.to_tokens(generator);
        self.expression.to_tokens(generator);
    }
}
