use crate::{ast::types::TupleType, Generator, ToTokens};

impl<'a> ToTokens for TupleType<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.types.to_tokens(generator);
        self.last.to_tokens(generator);
    }
}
