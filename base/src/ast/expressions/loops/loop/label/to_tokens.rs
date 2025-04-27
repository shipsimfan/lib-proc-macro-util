use crate::{ast::expressions::LoopLabel, Generator, ToTokens};

impl<'a> ToTokens for LoopLabel<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.quote.to_tokens(generator);
        self.name.to_tokens(generator);
        self.colon.to_tokens(generator);
    }
}
