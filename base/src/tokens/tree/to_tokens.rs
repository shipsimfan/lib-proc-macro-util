use crate::{tokens::TokenTree, Generator, ToTokens};

impl ToTokens for TokenTree {
    fn to_tokens(self, generator: &mut Generator) {
        generator.push(self)
    }
}
