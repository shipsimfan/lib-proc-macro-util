use crate::{ast::expressions::StructExprTuple, Generator, ToTokens};

impl<'a> ToTokens for StructExprTuple<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.first.to_tokens(generator);
        self.remaining.to_tokens(generator);
        self.last.to_tokens(generator);
    }
}
