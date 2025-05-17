use crate::{ast::expressions::StructExpression, Generator, ToTokens};

impl<'a> ToTokens for StructExpression<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.path.to_tokens(generator);
        self.kind.to_tokens(generator);
    }
}
