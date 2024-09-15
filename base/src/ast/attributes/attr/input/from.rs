use crate::{
    ast::{AttrInput, Expression},
    tokens::Group,
    Token,
};

impl<'a> From<&'a Group> for AttrInput<'a> {
    fn from(group: &'a Group) -> Self {
        AttrInput::Group(group)
    }
}

impl<'a> From<Group> for AttrInput<'a> {
    fn from(group: Group) -> Self {
        AttrInput::OwnedGroup(group)
    }
}

impl<'a, T: Into<Expression>> From<T> for AttrInput<'a> {
    fn from(value: T) -> Self {
        AttrInput::Expression(Token![=](), value.into())
    }
}

impl<'a, T: Into<Expression>> From<(Token![=], T)> for AttrInput<'a> {
    fn from(value: (Token![=], T)) -> Self {
        AttrInput::Expression(value.0, value.1.into())
    }
}
