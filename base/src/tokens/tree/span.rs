use crate::{tokens::TokenTree, Span};

impl TokenTree {
    /// Gets the span of this token tree
    pub fn span(&self) -> Span {
        match self {
            TokenTree::Group(group) => group.span,
            TokenTree::Identifier(identifier) => identifier.span(),
            TokenTree::Literal(literal) => literal.span(),
            TokenTree::Punctuation(punctuation) => punctuation.span(),
        }
    }
}
