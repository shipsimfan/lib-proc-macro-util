use crate::tokens::TokenTree;

impl std::fmt::Display for TokenTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenTree::Group(group) => group.fmt(f),
            TokenTree::Identifier(identifier) => identifier.fmt(f),
            TokenTree::Literal(literal) => literal.fmt(f),
            TokenTree::Punctuation(punctuation) => punctuation.fmt(f),
        }
    }
}
