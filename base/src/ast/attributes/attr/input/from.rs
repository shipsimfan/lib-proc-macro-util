use crate::{
    ast::{AttrInput, DelimTokenTree, Expression},
    tokens::Group,
    Token,
};

impl<'a> From<DelimTokenTree<'a>> for AttrInput<'a> {
    fn from(group: DelimTokenTree<'a>) -> Self {
        AttrInput::Group(group)
    }
}

impl<'a> From<&'a Group> for AttrInput<'a> {
    fn from(group: &'a Group) -> Self {
        AttrInput::Group(group.into())
    }
}

impl<'a> From<Group> for AttrInput<'a> {
    fn from(group: Group) -> Self {
        AttrInput::Group(group.into())
    }
}

impl<'a, T: Into<Expression<'a>>> From<T> for AttrInput<'a> {
    fn from(value: T) -> Self {
        AttrInput::Expression(Token![=](), value.into())
    }
}

impl<'a, T: Into<Expression<'a>>> From<(Token![=], T)> for AttrInput<'a> {
    fn from(value: (Token![=], T)) -> Self {
        AttrInput::Expression(value.0, value.1.into())
    }
}
