use crate::tokens::TokenTree;

impl From<proc_macro::TokenTree> for TokenTree {
    fn from(tree: proc_macro::TokenTree) -> Self {
        match tree {
            proc_macro::TokenTree::Group(group) => TokenTree::Group(group.into()),
            proc_macro::TokenTree::Ident(ident) => TokenTree::Identifier(ident.into()),
            proc_macro::TokenTree::Literal(literal) => TokenTree::Literal(literal.into()),
            proc_macro::TokenTree::Punct(punctuation) => TokenTree::Punctuation(punctuation.into()),
        }
    }
}
