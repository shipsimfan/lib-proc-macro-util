use crate::{ast::expressions::StructExprStruct, Generator, ToTokens};

impl<'a> ToTokens for StructExprStruct<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.first.to_tokens(generator);
        self.remaining.to_tokens(generator);
        self.base.to_tokens(generator);
    }
}
