use crate::{ast::expressions::ContinueExpression, Generator, ToTokens};

impl<'a> ToTokens for ContinueExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#continue.to_tokens(generator);
        self.lifetime.to_tokens(generator);
    }
}
