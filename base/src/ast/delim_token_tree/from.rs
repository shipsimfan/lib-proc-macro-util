use crate::{ast::DelimTokenTree, tokens::Group};

impl<'a> From<&'a Group> for DelimTokenTree<'a> {
    fn from(group: &'a Group) -> Self {
        DelimTokenTree::Borrowed(group)
    }
}

impl<'a> From<Group> for DelimTokenTree<'a> {
    fn from(group: Group) -> Self {
        DelimTokenTree::Owned(group)
    }
}
