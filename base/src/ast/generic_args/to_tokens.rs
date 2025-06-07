use crate::{ast::GenericArgs, Generator, ToTokens};

impl<'a> ToTokens for GenericArgs<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.open.to_tokens(generator);
        self.args.to_tokens(generator);
        self.last_arg.to_tokens(generator);
        self.last_comma.to_tokens(generator);
        self.close.to_tokens(generator);
    }
}
