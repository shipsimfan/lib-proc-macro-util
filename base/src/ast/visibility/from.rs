use crate::{
    ast::{SimplePath, Visibility},
    Token,
};

use super::VisibilityScope;

impl<'a> From<()> for Visibility<'a> {
    fn from(_: ()) -> Self {
        Visibility::new_pub()
    }
}

impl<'a> From<Token![pub]> for Visibility<'a> {
    fn from(r#pub: Token![pub]) -> Self {
        Visibility::new_raw(r#pub, None)
    }
}

impl<'a> From<Token![crate]> for Visibility<'a> {
    fn from(krate: Token![crate]) -> Self {
        Visibility::new(krate)
    }
}

impl<'a> From<(Token![pub], Token![crate])> for Visibility<'a> {
    fn from(value: (Token![pub], Token![crate])) -> Self {
        Visibility::new_raw(value.0, Some(value.1.into()))
    }
}

impl<'a> From<Token![self]> for Visibility<'a> {
    fn from(_self: Token![self]) -> Self {
        Visibility::new(_self)
    }
}

impl<'a> From<(Token![pub], Token![self])> for Visibility<'a> {
    fn from(value: (Token![pub], Token![self])) -> Self {
        Visibility::new_raw(value.0, Some(value.1.into()))
    }
}

impl<'a> From<Token![super]> for Visibility<'a> {
    fn from(_super: Token![super]) -> Self {
        Visibility::new(_super)
    }
}

impl<'a> From<(Token![pub], Token![super])> for Visibility<'a> {
    fn from(value: (Token![pub], Token![super])) -> Self {
        Visibility::new_raw(value.0, Some(value.1.into()))
    }
}

impl<'a> From<SimplePath<'a>> for Visibility<'a> {
    fn from(path: SimplePath<'a>) -> Self {
        Visibility::new_path(path)
    }
}

impl<'a> From<(Token![pub], SimplePath<'a>)> for Visibility<'a> {
    fn from(value: (Token![pub], SimplePath<'a>)) -> Self {
        Visibility::new_raw(value.0, Some(value.1.into()))
    }
}

impl<'a, T: Into<SimplePath<'a>>> From<(Token![in], T)> for Visibility<'a> {
    fn from(value: (Token![in], T)) -> Self {
        Visibility::new_raw(
            Token![pub](),
            Some(VisibilityScope::Path(value.0, value.1.into())),
        )
    }
}

impl<'a, T: Into<SimplePath<'a>>> From<(Token![pub], Token![in], T)> for Visibility<'a> {
    fn from(value: (Token![pub], Token![in], T)) -> Self {
        Visibility::new_raw(
            value.0,
            Some(VisibilityScope::Path(value.1, value.2.into())),
        )
    }
}
