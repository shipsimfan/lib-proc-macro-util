use crate::{ast::expressions::ConstBlockExpression, Generator, ToTokens};

impl<'a> ToTokens for ConstBlockExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.r#const.to_tokens(generator);
        self.block.to_tokens(generator);
    }
}
