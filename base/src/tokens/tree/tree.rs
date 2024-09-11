use crate::{
    tokens::{Group, Identifier, Literal, Punctuation},
    Generator, ToTokens,
};

/// One token or tree
#[derive(Debug, Clone)]
pub enum TokenTree {
    /// A delimited group of tokens
    Group(Group),

    /// An identifier
    Identifier(Identifier),

    /// A string or numeric literal
    Literal(Literal),

    /// A single character of punctuation
    Punctuation(Punctuation),
}

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

impl ToTokens for TokenTree {
    fn to_tokens(self, generator: &mut Generator) {
        generator.push(self)
    }
}
