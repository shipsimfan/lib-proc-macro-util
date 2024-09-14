use proc_macro_util_base::{Generator, ToTokens};

impl<'a> ToTokens for super::ToTokens<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        for token in self.tokens {
            token.to_tokens(generator, self.generator);
        }
    }
}
