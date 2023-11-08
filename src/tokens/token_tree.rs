use crate::tokens::{Group, Ident, Literal, Punctuation};

#[derive(Clone)]
pub enum TokenTree<'a> {
    Ident(Ident),
    Punctuation(Punctuation),
    Literal(Literal),
    Group(Group<'a>),
}

impl<'a> Into<proc_macro::TokenTree> for TokenTree<'a> {
    fn into(self) -> proc_macro::TokenTree {
        match self {
            TokenTree::Ident(ident) => proc_macro::TokenTree::Ident(ident),
            TokenTree::Punctuation(punctuation) => proc_macro::TokenTree::Punct(punctuation),
            TokenTree::Literal(literal) => proc_macro::TokenTree::Literal(literal),
            TokenTree::Group(group) => proc_macro::TokenTree::Group(group.into()),
        }
    }
}
