use crate::{ast::expressions::DereferenceExpression, Generator, ToTokens};

impl<'a> ToTokens for DereferenceExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.asterick.to_tokens(generator);
        self.expression.to_tokens(generator);
    }
}
