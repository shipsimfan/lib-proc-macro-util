use crate::{ast::expressions::PredicateLoopExpression, Generator, ToTokens};

impl<'a> ToTokens for PredicateLoopExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#while.to_tokens(generator);
        self.r#let.to_tokens(generator);
        self.condition.to_tokens(generator);
        self.block.to_tokens(generator);
    }
}
