use crate::tokens::{Identifier, Literal, OwnedGroup, Punctuation};

#[derive(Debug)]
pub(crate) enum OwnedTokenTree {
    Group(OwnedGroup),
    Identifier(Identifier),
    Literal(Literal),
    Punctuation(Punctuation),
}

impl From<proc_macro::TokenTree> for OwnedTokenTree {
    fn from(token: proc_macro::TokenTree) -> Self {
        match token {
            proc_macro::TokenTree::Group(group) => OwnedTokenTree::Group(group.into()),
            proc_macro::TokenTree::Ident(identifier) => {
                OwnedTokenTree::Identifier(identifier.into())
            }
            proc_macro::TokenTree::Literal(literal) => OwnedTokenTree::Literal(literal.into()),
            proc_macro::TokenTree::Punct(punctuation) => {
                OwnedTokenTree::Punctuation(punctuation.into())
            }
        }
    }
}

impl Into<proc_macro::TokenTree> for OwnedTokenTree {
    fn into(self) -> proc_macro::TokenTree {
        match self {
            OwnedTokenTree::Group(group) => proc_macro::TokenTree::Group(group.into()),
            OwnedTokenTree::Identifier(identifier) => {
                proc_macro::TokenTree::Ident(identifier.into())
            }
            OwnedTokenTree::Literal(literal) => proc_macro::TokenTree::Literal(literal.into()),
            OwnedTokenTree::Punctuation(punctuation) => {
                proc_macro::TokenTree::Punct(punctuation.into())
            }
        }
    }
}
