use crate::{
    ast::{SimplePath, VisibilityScope},
    Token,
};

impl<'a> From<Token![crate]> for VisibilityScope<'a> {
    fn from(krate: Token![crate]) -> Self {
        VisibilityScope::Crate(krate)
    }
}

impl<'a> From<Token![self]> for VisibilityScope<'a> {
    fn from(_self: Token![self]) -> Self {
        VisibilityScope::_Self(_self)
    }
}

impl<'a> From<Token![super]> for VisibilityScope<'a> {
    fn from(_super: Token![super]) -> Self {
        VisibilityScope::Super(_super)
    }
}

impl<'a> From<SimplePath<'a>> for VisibilityScope<'a> {
    fn from(path: SimplePath<'a>) -> Self {
        VisibilityScope::new_path(path)
    }
}

impl<'a, T: Into<SimplePath<'a>>> From<(Token![in], T)> for VisibilityScope<'a> {
    fn from(value: (Token![in], T)) -> Self {
        VisibilityScope::Path(value.0, value.1.into())
    }
}
