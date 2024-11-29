use crate::ast::AttrInput;
use std::borrow::Cow;

impl<'a> AttrInput<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> AttrInput<'static> {
        match self {
            AttrInput::Expression(eq, expression) => {
                AttrInput::Expression(eq, expression.into_static())
            }
            AttrInput::Group(group) => AttrInput::Group(Cow::Owned(match group {
                Cow::Borrowed(borrowed) => borrowed.clone(),
                Cow::Owned(owned) => owned,
            })),
        }
    }
}
