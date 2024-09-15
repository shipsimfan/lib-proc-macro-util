use crate::ast::AttrInput;

impl<'a> std::fmt::Display for AttrInput<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttrInput::Group(group) => group.fmt(f),
            AttrInput::OwnedGroup(group) => group.fmt(f),
            AttrInput::Expression(_, expression) => {
                f.write_str(" = ")?;
                expression.fmt(f)
            }
        }
    }
}
