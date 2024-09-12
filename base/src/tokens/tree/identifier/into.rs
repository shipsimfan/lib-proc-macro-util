use crate::tokens::{Identifier, TokenTree};

impl Into<proc_macro::Ident> for Identifier {
    fn into(self) -> proc_macro::Ident {
        self.0
    }
}

impl Into<TokenTree> for Identifier {
    fn into(self) -> TokenTree {
        TokenTree::Identifier(self)
    }
}

impl Into<proc_macro::TokenTree> for Identifier {
    fn into(self) -> proc_macro::TokenTree {
        proc_macro::TokenTree::Ident(self.0)
    }
}
