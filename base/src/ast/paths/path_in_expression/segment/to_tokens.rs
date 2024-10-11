use crate::{ast::PathExprSegment, Generator, ToTokens};

impl<'a> ToTokens for PathExprSegment<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        self.ident.to_tokens(generator);
        self.generic_args.to_tokens(generator);
    }
}
