use crate::{
    ast::{Attr, OuterAttribute},
    Token,
};

impl<'a> OuterAttribute<'a> {
    /// Creates a new [`OuterAttribute`]
    pub const fn new_raw(hash: Token![#], attr: Attr<'a>) -> Self {
        OuterAttribute { hash, attr }
    }

    /// Creates a new [`OuterAttribute`] from `attr`
    pub fn new<T: Into<Attr<'a>>>(attr: T) -> Self {
        OuterAttribute::new_raw(Token![#](), attr.into())
    }
}
