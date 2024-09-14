use crate::tokens::TokenTree;

impl Into<proc_macro::TokenTree> for TokenTree {
    fn into(self) -> proc_macro::TokenTree {
        match self {
            TokenTree::Group(group) => proc_macro::TokenTree::Group(group.into()),
            TokenTree::Identifier(ident) => proc_macro::TokenTree::Ident(ident.into()),
            TokenTree::Literal(literal) => proc_macro::TokenTree::Literal(literal.into()),
            TokenTree::Punctuation(punctuation) => proc_macro::TokenTree::Punct(punctuation.into()),
        }
    }
}
