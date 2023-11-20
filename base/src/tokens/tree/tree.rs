use crate::tokens::{Group, Identifier, Literal, OwnedTokenTree, Punctuation};

/// One token or tree
#[derive(Clone)]
pub enum TokenTree<'a> {
    /// A delimited group of tokens
    Group(Group<'a>),

    /// An identifier
    Identifier(Identifier),

    /// A string or numeric literal
    Literal(Literal),

    /// A single character of punctuation
    Punctuation(Punctuation),
}

impl<'a> From<&'a OwnedTokenTree> for TokenTree<'a> {
    fn from(token: &'a OwnedTokenTree) -> Self {
        match token {
            OwnedTokenTree::Group(group) => TokenTree::Group(group.into()),
            OwnedTokenTree::Identifier(identifier) => TokenTree::Identifier(identifier.clone()),
            OwnedTokenTree::Literal(literal) => TokenTree::Literal(literal.clone()),
            OwnedTokenTree::Punctuation(punctuation) => TokenTree::Punctuation(punctuation.clone()),
        }
    }
}

impl<'a> Into<OwnedTokenTree> for TokenTree<'a> {
    fn into(self) -> OwnedTokenTree {
        match self {
            TokenTree::Group(group) => OwnedTokenTree::Group(group.into()),
            TokenTree::Identifier(identifier) => OwnedTokenTree::Identifier(identifier.into()),
            TokenTree::Literal(literal) => OwnedTokenTree::Literal(literal.into()),
            TokenTree::Punctuation(punctuation) => OwnedTokenTree::Punctuation(punctuation.into()),
        }
    }
}
