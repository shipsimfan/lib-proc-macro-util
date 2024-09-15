use crate::{
    ast::{Attr, InnerAttribute},
    Token,
};

impl<'a> InnerAttribute<'a> {
    /// Creates a new [`InnerAttribute`]
    pub const fn new_raw(hash: Token![#], exclamation: Token![!], attr: Attr<'a>) -> Self {
        InnerAttribute {
            hash,
            exclamation,
            attr,
        }
    }

    /// Creates a new [`InnerAttribute`] from `attr`
    pub fn new<T: Into<Attr<'a>>>(attr: T) -> Self {
        InnerAttribute::new_raw(Token![#](), Token![!](), attr.into())
    }
}
