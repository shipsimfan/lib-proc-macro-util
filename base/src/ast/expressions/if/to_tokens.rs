use crate::{ast::expressions::IfExpression, Generator, ToTokens};

impl<'a> ToTokens for IfExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#if.to_tokens(generator);
        self.r#let.to_tokens(generator);
        self.condition.to_tokens(generator);
        self.block.to_tokens(generator);
        self.r#else.to_tokens(generator);
    }
}
