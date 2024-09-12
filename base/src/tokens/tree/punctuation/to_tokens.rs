use crate::tokens::Punctuation;

impl ToTokens for Punctuation {
    fn to_tokens(self, generator: &mut Generator) {
        TokenTree::Punctuation(self).to_tokens(generator);
    }
}
