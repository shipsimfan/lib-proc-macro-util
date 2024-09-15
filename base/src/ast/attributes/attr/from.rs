use crate::ast::{Attr, AttrInput, SimplePath};

impl<'a, T: Into<SimplePath<'a>>> From<T> for Attr<'a> {
    fn from(path: T) -> Self {
        Attr::new_raw(path.into(), None)
    }
}

impl<'a> From<(SimplePath<'a>, AttrInput<'a>)> for Attr<'a> {
    fn from(value: (SimplePath<'a>, AttrInput<'a>)) -> Self {
        Attr::new_raw(value.0, Some(value.1))
    }
}

impl<'a> From<(SimplePath<'a>, Option<AttrInput<'a>>)> for Attr<'a> {
    fn from(value: (SimplePath<'a>, Option<AttrInput<'a>>)) -> Self {
        Attr::new_raw(value.0, value.1)
    }
}
