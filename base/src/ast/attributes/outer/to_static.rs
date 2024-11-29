use crate::ast::OuterAttribute;

impl<'a> OuterAttribute<'a> {
    /// Takes ownership of any borrowed elements and converts the lifetime to `'static`
    pub fn into_static(self) -> OuterAttribute<'static> {
        OuterAttribute {
            hash: self.hash,
            attr: self.attr.into_static(),
        }
    }
}
