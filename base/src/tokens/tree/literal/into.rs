use crate::tokens::{Literal, TokenTree};

impl Into<proc_macro::Literal> for Literal {
    fn into(self) -> proc_macro::Literal {
        self.0
    }
}

impl Into<TokenTree> for Literal {
    fn into(self) -> TokenTree {
        TokenTree::Literal(self)
    }
}
