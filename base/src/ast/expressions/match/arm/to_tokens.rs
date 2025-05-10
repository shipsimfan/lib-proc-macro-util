use crate::{ast::expressions::MatchArm, Generator, ToTokens};

impl<'a> ToTokens for MatchArm<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.attributes.to_tokens(generator);
        self.pattern.to_tokens(generator);
        self.guard.to_tokens(generator);
    }
}
