use crate::{ast::DelimTokenTree, tokens::Group};
use std::ops::Deref;

impl<'a> Deref for DelimTokenTree<'a> {
    type Target = Group;

    fn deref(&self) -> &Self::Target {
        match self {
            DelimTokenTree::Borrowed(group) => group,
            DelimTokenTree::Owned(group) => group,
        }
    }
}
