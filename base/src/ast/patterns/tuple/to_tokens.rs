use crate::{ast::patterns::TuplePattern, Generator, ToTokens};

impl<'a> ToTokens for TuplePattern<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.items.to_tokens(&mut generator.group_bracket());
    }
}
