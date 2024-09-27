use crate::{ast::GenericArgsBinding, Generator, ToTokens};

impl<'a> ToTokens for GenericArgsBinding<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.identifier.to_tokens(generator);
        self.args.to_tokens(generator);
        self.equals.to_tokens(generator);
        self.value.to_tokens(generator);
    }
}
