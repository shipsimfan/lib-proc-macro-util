use crate::{ast::expressions::CallParams, Generator, ToTokens};

impl<'a> ToTokens for CallParams<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let mut group = generator.group_parenthesis();
        self.first.to_tokens(&mut group);
        self.remaining.to_tokens(&mut group);
        self.last.to_tokens(&mut group);
    }
}
