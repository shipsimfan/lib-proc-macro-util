use crate::{ast::expressions::AsyncBlockExpression, Generator, ToTokens};

impl<'a> ToTokens for AsyncBlockExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#async.to_tokens(generator);
        self.r#move.to_tokens(generator);
        self.block.to_tokens(generator);
    }
}
