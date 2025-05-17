use crate::{ast::expressions::StructExprField, Generator, ToTokens};

impl<'a> ToTokens for StructExprField<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.attributes.to_tokens(generator);
        self.name.to_tokens(generator);
        self.expression.to_tokens(generator);
    }
}
