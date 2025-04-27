use crate::{ast::patterns::TupleStructPattern, Generator, ToTokens};

impl<'a> ToTokens for TupleStructPattern<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.path.to_tokens(generator);
        self.items.to_tokens(&mut generator.group_bracket());
    }
}
