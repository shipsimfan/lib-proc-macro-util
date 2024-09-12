use crate::{
    tokens::{Literal, TokenTree},
    Generator, ToTokens,
};

impl ToTokens for Literal {
    fn to_tokens(self, generator: &mut Generator) {
        TokenTree::Literal(self).to_tokens(generator)
    }
}
