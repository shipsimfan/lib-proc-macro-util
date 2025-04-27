use crate::{ast::expressions::IteratorLoopExpression, Generator, ToTokens};

impl<'a> ToTokens for IteratorLoopExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#for.to_tokens(generator);
        self.pattern.to_tokens(generator);
        self.r#in.to_tokens(generator);
        self.iterator.to_tokens(generator);
        self.block.to_tokens(generator);
    }
}
