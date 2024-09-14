use crate::tokens::{Punctuation, TokenTree};

impl Into<proc_macro::Punct> for Punctuation {
    fn into(self) -> proc_macro::Punct {
        self.0
    }
}

impl Into<TokenTree> for Punctuation {
    fn into(self) -> TokenTree {
        TokenTree::Punctuation(self)
    }
}

impl Into<char> for Punctuation {
    fn into(self) -> char {
        self.as_char()
    }
}
