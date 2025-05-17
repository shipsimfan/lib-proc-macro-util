use crate::{ast::expressions::StructBase, Generator, ToTokens};

impl<'a> ToTokens for StructBase<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.dots.to_tokens(generator);
        self.expression.to_tokens(generator);
    }
}
