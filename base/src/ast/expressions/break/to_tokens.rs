use crate::{ast::expressions::BreakExpression, Generator, ToTokens};

impl<'a> ToTokens for BreakExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#break.to_tokens(generator);
        self.lifetime.to_tokens(generator);
        self.expression.to_tokens(generator);
    }
}
