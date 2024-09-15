use crate::{
    ast::{Attr, InnerAttribute},
    Token,
};

impl<'a, T: Into<Attr<'a>>> From<T> for InnerAttribute<'a> {
    fn from(value: T) -> Self {
        InnerAttribute::new(value)
    }
}

impl<'a, T: Into<Attr<'a>>> From<(Token![#], Token![!], T)> for InnerAttribute<'a> {
    fn from(value: (Token![#], Token![!], T)) -> Self {
        InnerAttribute::new_raw(value.0, value.1, value.2.into())
    }
}
