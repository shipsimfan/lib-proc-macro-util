use crate::{
    ast::{Attr, OuterAttribute},
    Token,
};

impl<'a, T: Into<Attr<'a>>> From<T> for OuterAttribute<'a> {
    fn from(value: T) -> Self {
        OuterAttribute::new(value)
    }
}

impl<'a, T: Into<Attr<'a>>> From<(Token![#], T)> for OuterAttribute<'a> {
    fn from(value: (Token![#], T)) -> Self {
        OuterAttribute::new_raw(value.0, value.1.into())
    }
}
