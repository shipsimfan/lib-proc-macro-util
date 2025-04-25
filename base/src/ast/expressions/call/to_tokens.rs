use crate::{ast::expressions::CallExpression, Generator, ToTokens};

impl<'a> ToTokens for CallExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.function.to_tokens(generator);
        self.parameters.to_tokens(generator);
    }
}
