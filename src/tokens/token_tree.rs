use crate::tokens::{Group, Ident, Literal, Punctuation};

#[derive(Clone)]
pub enum TokenTree<'a> {
    Ident(Ident),
    Punctuation(Punctuation),
    Literal(Literal),
    Group(Group<'a>),
}
