use crate::{ast::expressions::InfiniteLoopExpression, Generator, ToTokens};

impl<'a> ToTokens for InfiniteLoopExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#loop.to_tokens(generator);
        self.block.to_tokens(generator);
    }
}
