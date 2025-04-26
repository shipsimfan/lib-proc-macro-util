use crate::{ast::expressions::MethodCallExpression, Generator, ToTokens};

impl<'a> ToTokens for MethodCallExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.function.to_tokens(generator);
        self.dot.to_tokens(generator);
        self.name.to_tokens(generator);
        self.parameters.to_tokens(generator);
    }
}
