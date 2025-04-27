use crate::{ast::patterns::GroupedPattern, Generator, ToTokens};

impl<'a> ToTokens for GroupedPattern<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.pattern.to_tokens(&mut generator.group_parenthesis());
    }
}
