use crate::{ast::DelimTokenTree, Generator, ToTokens};

impl<'a> ToTokens for DelimTokenTree<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            DelimTokenTree::Borrowed(group) => group.clone().to_tokens(generator),
            DelimTokenTree::Owned(group) => group.to_tokens(generator),
        }
    }
}
