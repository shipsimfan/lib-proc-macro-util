use crate::{ast::GenericParams, Generator, ToTokens};

impl<'a> ToTokens for GenericParams<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.open.to_tokens(generator);
        self.params.to_tokens(generator);
        self.last_param.to_tokens(generator);
        self.last_comma.to_tokens(generator);
        self.close.to_tokens(generator);
    }
}
