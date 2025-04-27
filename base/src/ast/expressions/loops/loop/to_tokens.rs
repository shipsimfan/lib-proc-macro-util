use crate::{ast::expressions::LoopExpression, Generator, ToTokens};

impl<'a> ToTokens for LoopExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.label.to_tokens(generator);
        self.kind.to_tokens(generator);
    }
}
