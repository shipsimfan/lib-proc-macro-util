use crate::{ast::expressions::UnsafeBlockExpression, Generator, ToTokens};

impl<'a> ToTokens for UnsafeBlockExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#unsafe.to_tokens(generator);
        self.block.to_tokens(generator);
    }
}
