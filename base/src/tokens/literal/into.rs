use crate::tokens::{Literal, TokenTree};
use std::borrow::Cow;

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

impl Into<proc_macro::TokenTree> for Literal {
    fn into(self) -> proc_macro::TokenTree {
        proc_macro::TokenTree::Literal(self.0)
    }
}

impl<'a> Into<Cow<'a, Literal>> for Literal {
    fn into(self) -> Cow<'a, Literal> {
        Cow::Owned(self)
    }
}

impl<'a> Into<Cow<'a, Literal>> for &'a Literal {
    fn into(self) -> Cow<'a, Literal> {
        Cow::Borrowed(self)
    }
}
