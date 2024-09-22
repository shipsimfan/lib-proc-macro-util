use crate::{ast::MacroInvocation, Generator, ToTokens};

impl<'a> ToTokens for MacroInvocation<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.path.to_tokens(generator);
        self.exclamation.to_tokens(generator);
        self.group.to_tokens(generator);
    }
}
