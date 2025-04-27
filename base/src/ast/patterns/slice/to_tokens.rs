use crate::{ast::patterns::SlicePattern, Generator, ToTokens};

impl<'a> ToTokens for SlicePattern<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.items.to_tokens(&mut generator.group_bracket());
    }
}
