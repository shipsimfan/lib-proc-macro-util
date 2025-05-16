use crate::{ast::expressions::AwaitExpression, Generator, ToTokens};

impl<'a> ToTokens for AwaitExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.expression.to_tokens(generator);
        self.dot.to_tokens(generator);
        self.r#await.to_tokens(generator);
    }
}
